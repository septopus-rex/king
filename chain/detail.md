# Septopus整体架构

* 系统由`入口合约`和`子合约`构成，合约都独立部署，通过Solana的CPI方式进行请求。
* `入口合约`命名为`Entry`，用于统一接受多系统的请求，同时也验证子合约是否合法。

## Entry合约

* Entry合约外部请求的方法列表

|  合约方法   | 分类  | 功能描述  | 参数说明  | 签名人  |
|  :----  | :----  | :---- | :---- | :---- |
|  init  | 基础 | 初始化Entry,建立必要的账户，仅运行一次 | {king:KING_ACCOUNT} | KING_ACCOUNT |
|  reg  | 子合约管理 | 注册一个子合约，保存必须的信息 | {address:SUB_PROGRAM_ACCOUNT} | KING_ACCOUNT |
|  remove  | 子合约管理 | 移除一个子合约 | {index:SUB_PROGRAM_INDEX} | KING_ACCOUNT |
|  run  | 功能 | 请求子合约 | { index:SUB_PROGRAM_INDEX,params:PARAMETERS[] } | KING_ACCOUNT |

### Entry功能实现

* 初始化系统。执行Entry.init(KING_ACCOUNT)，设置整个系统。创建ENTRY_TASK_ACCOUT，用来执行子合约的任务。
* 增加子合约。执行Entry.reg(SUB_PROGRAM_ACCOUNT)，对SUB_PROGRAM_ACCOUNT进行白名单处理，并给予编号index。
* 移除子合约。执行Entry.remove(SUB_PROGRAM_INDEX)，将SUB_PROGRAM_INDEX对应的子合约设置成`不可用`。
* 执行子合约。执行Entry.run(SUB_PROGRAM_INDEX, PARAMETERS[])，执行SUB_PROGRAM_INDEX对应的子合约。需要检查子合约的合法性。如果执行的是King.approve，得到NEW_KING_ACCOUNT时候，修改系统的KING_ACCOUNT。

## King合约

* King合约外部请求的方法列表

|  合约方法   | 分类  | 功能描述  | 参数说明  | 签名人  |
|  :----  | :----  | :---- | :---- | :---- |
|  init  | 基础 | 初始化King系统，建立必要的账号 | {config:"JSON_SETTING_STRING"} | KING_ACCOUNT |
|  update  | 基础 | 修改King系统配置 | {config:"JSON_SETTING_STRING"} | KING_ACCOUNT |
|  launch  | 基础 | 系统开始去中心化运行，无法再进行随意添加 | null | KING_ACCOUNT |
|  replace  | 基础 | 指定更新King的系统配置 | {key:"SETTING_KEY",value:"SETTING_VALUE"} | ENTRY_TASK_ACCOUT |
|  pool  | 乐透选取 | 加入选取池 |  | Anyone |
|  approve  | 乐透选取 | 验证选取结果，100万次的sha256循环计算 |  | Anyone |
|  apply  | 循环 | 申请一项审核，需要king来进行处理 | {detail:"IPFS_ADDRESS",action:"ACTION_JSON_STRING"} | SUB_PROGRAM |
|  review  | 循环 | King进行审核的操作，并附带结果 | {index:"TASK_INDEX",result:"ACCEPT_OR_REJECT"} | King |
|  claim  | 循环 | King申请费用的操作 | {index:"TASK_INDEX"} | King |
|  impeach  | 异常处理 | 弹劾King的操作 |  | ENTRY_TASK_ACCOUT |

### King子合约功能实现

#### 子合约配置

* 初始化系统。执行King.init(JSON_SETTING_STRING)，设置整个系统。设置ENTRY_TASK_ACCOUT，用于验证执行任务的账号是否合法。
* 修改系统配置。执行King.update(JSON_SETTING_STRING)，修改系统的配置。
* 系统上线。执行King.launch()，关闭King修改权限，只能通过提案来进行修改。
* 受限更新。执行King.replace(SETTING_KEY,SETTING_VALUE)，单独修改King系统的配置。需要在launch之后，才可以执行。

#### King选取实现

* 加入选取池。执行King.pool()，加请求者加入选取池。
  1. 选取按照链上时间进行启动，时间间隔在Setting["lottery_interval"]取得，每次选取，建立独立的pool来保存数据。
  2. 如果加入的时候，已经开启下一轮，则记录这次的SLOT_HASH作为选取验证的SEED。
  3. 成功执行入池，需要支付Setting["lottery_fee"]给Setting["treasure_holder"]。
* 验证乐透选中King。执行King.approve()，进行循环计算验证King。当获取到该轮选取的SEED之后，就可以开始计算，以下是验证算法的说明。
  1. 当第1次计算时候，采用SEED来循环1万次SHA256计算，保存结果，供下一次计算。
  2. 接下来的循环，判断是否达到Setting["lottery_limit"]的次数，每次最多1万次SHA256计算。
  3. 当SHA256循环次数达到Setting["lottery_limit"]的次数时，用此结果Hash来确定新的King。
  4. 选取的方法是 FINAL_HASH % LOTTREY_POOL.length，即根据最终Hash从选取池里取出KING_ACCOUNT，并返回给`Entry合约`，用于设置整个系统的King。
  5. 确认King之后，设置King可领薪水的数据结构。根据Setting["king_benifit_loop"]在Setting["lottery_interval"]的链上时间进行均不布。需向前设置Setting["king_benifit_advance"]，防止出现King领薪水被下一任King阻止的问题，供之后King.claim()来使用。
  6. 更新King的历史记录，记录上一位King的结束链上时间，加入选中的King，设置其开始的时间。

#### King审批实现

* 创建审核内容。执行King.apply()，创建一个需要King进行审核的任务。
  1. 审核内容就是一个Task，可以执行对系统的修改，采用统一的数据结构。
  2. 审核内容只能由其他子合约生成，King审核通过的话，由Entry来进行执行。
* King审核。执行King.review(TASK_INDEX,true|false)，给一个结果。该命令只能由当前的King来执行。

#### King经费申请

* 获取King的薪水。
  1. 执行King.claim()，将查询链上申请记录，如可领取，创建资金分发的审核(由treasure子合约实现，Treasure.pay(TO_ACCOUNT,AMOUNT, TOKEN))，获取到BENIFIT_TASK_INDEX。
  2. 执行King.approve(TASK_INDEX，true)，来获取到费用。

#### 弹劾King的操作

* 系统发起了对King的弹劾。该方法King.impeach()只能由ENTRY_TASK_ACCOUT发起。执行后，将系统的King设置为空，等待下一个King的产生。

### 数据结构

* King系统的设置如下：
  
```JSON
    {
        "treasure_holder": "SOLANA_ACCOUNT",
        "lottery_interval": 403200,
        "system_start": 289403200,
        "lottery_limit": 1000000,
        "lottery_fee": 100000,
        "king_benifit_loop": 201600,
        "king_benifit_advance": 28800,
        "king_benifit_amount": 600,
        "king_benifit_token": "SOL",
    }
```

* King审批的数据结构如下：
  
```JSON
    {
        "index": 17,
        "detail": "IPFS_ADDRESS",
        "stamp": 289403200,
        "action":[
            {
                "module":"SUB_PROGRAM_INDEX",
                "method":"METHOD_OF_SUB_PROGRAM",
                "parameter":"",
            },
        ],
        "result":{
            "approved": true,
            "king":"APPROVE_KING_ACCOUNT",
            "stamp":289408900,
        }
    }
```