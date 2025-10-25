# Septopus整体架构

## 合约入口

* 使用Solana的CPI结构来组织`Septopus`的所有合约，使用单一入口。
* 使用Solana的`代理模式`来实现，主合约为`Entry`。
* 子合约通过验证`Entry`合约传递过来的验证PDA账号，来确认请求来自`Entry`的请求。

## 合约结构

* `Septopus`的功能，合约拆分情况如下：

|  合约名称   | 功能描述  | 详情  | 所属模块  |
|  ----  | ----  | ----  | ----  |
|  Rules  | Rules的数据、Rules的讨论、Rules的修改 |  | Rules Center |
|  King  |  King的乐透选取、King的日常签到、King的审批签署、King的支付审核 |  | King Center |
|  Project  |  Project管理 |  | King Center |
|  Group  |  多签钱包的管理 |  | King Center |
|  Token  |  项目token的管理（创建、分发、锁定） |  | King Center |
|  Treasure  |  国库的资金管理 |  | King Center |
|  AI  |  AI的审核、AI的部署 |  | AI Center |
|  Adjunct  |  Adjunct创建、Adjunct更新等 |  | Meta Septopus |
|  Resoure  |  Resouce创建、Resouce更新、Resouce举报 |  | Meta Septopus |
|  World  |  World的拍卖、World的配置、World的销售状态 |  | Meta Septopus |
|  Block  |  数据保存、Block交易、Block举报、Block禁显、Block申请恢复、Block恢复 |  | Meta Septopus, 拟使用cNFT来实现 |

### Rules

* Rules数据组织涉及到的PDA账号
  
|  账户名   | 是否King  | Seeds  | 功能说明  | 数据结构  |
|  ----  | ----  | ---- | ---- | ---- |
|  rules_setting  | ✅ | ["RULES_SETTING"] | 配置运行的参数，例如各种条件的百分比 |  |
|  rules_index  |  | ["RULES_INDEX"] | 条目的索引值 | u32 |
|  rules_data  |  | ["RULES_DATA",rules_index] | 条目的索引值 | {raw:"",status:0} |
|  rules_comment  |  | ["RULES_COMMENT",rules_index] | 条目的索引值 | comment[] |
|  rules_vote  |  | ["RULES_VOTE",rules_index] | 针对rule的投票，每条只能发起一次 | {} |

* Rules合约外部请求的方法列表
  
|  合约方法   | 分类  | 功能描述  | 参数说明  | 签名人  |
|  ----  | ----  | ---- | ---- | ---- |
|  init  | 管理 | 初始化Rules系统，建立必要的账号 | null | King |
|  config  | 管理 | 配置运行的参数，例如各种条件的百分比 |  | King |
|  update  | 管理 | 修改Rules的配置，需经King审核 | JSON | King |
|  launch  | 管理 | 系统开始去中心化运行，无法再进行随意添加 |  | King |
|  add  | 循环 | 添加1条rules |  | King/Anyone |
|  abandon  | 循环 | 废弃投票通过的rule |  | Anyone |
|  start  | 循环 | 发起对新的rule进行投票 |  | Anyone |
|  vote  | 循环 | 针对rule进行投票的动作 |  | Anyone |
|  comment  | 循环 | 对任何一条rule进行评论 |  | Anyone |
|  accept  | 循环 | 接受一条新的rule |  | Anyone |

### King合约

* King数据组织涉及到的PDA账号

* King合约外部请求的方法列表

|  合约方法   | 分类  | 功能描述  | 参数说明  | 签名人  |
|  ----  | ----  | ---- | ---- | ---- |
|  init  | 管理 | 初始化King系统，建立必要的账号 | null | King |
|  config  | 管理 | 配置运行的参数，例如各种条件的百分比 | {agent:"Multi_sign_wallet"} | King |
|  update  | 管理 | 修改Treasure的配置 | JSON | King |
|  launch  | 管理 | 系统开始去中心化运行，无法再进行随意添加 |  | Anyone |
|  lottery  | 乐透选取 | 启动King的随机选取过程 |  | King |
|  pool  | 乐透选取 | 加入选取池 |  | Anyone |
|  approve  | 乐透选取 | 验证选取结果，100万次的sha256循环计算 |  | Anyone |
|  apply  | 循环 | 申请一项审核，需要king来进行处理 |  | Contract |
|  review  | 循环 | King进行审核的操作，并附带结果 |  | King |
|  abandon  | 循环 | King放弃自己位置的操作，会重新进入乐透选取 |  | Anyone |
|  claim  | 循环 | King申请费用的操作 |  | King |
|  impeach  | 循环 | 弹劾King的操作 |  | Anyone |

### Project合约

* Project数据组织涉及到的PDA账号

|  账户名   | 是否King  | Seeds  | 功能说明  | 数据结构  |
|  ----  | ----  | ---- | ---- | ---- |
|  project_setting  | ✅ | ["PROJECT_SETTING"] | 配置运行的参数，例如各种条件的百分比 |  |

* Project合约外部请求的方法列表

|  合约方法   | 分类  | 功能描述  | 参数说明  | 签名人  |
|  ----  | ----  | ---- | ---- | ---- |
|  init  | 管理 | 初始化Project系统，建立必要的账号 | null | King |
|  config  | 管理 | 配置Project运行的参数，例如各种条件的百分比 | JSON | King |
|  update  | 管理 | 修改Project的配置 | JSON | King |
|  launch  | 管理 | 系统开始去中心化运行，无法再进行随意添加 |  | King |
|  add  | 循环 | 添加1个Project |  | Anyone |
|  apply  | 循环 | Project进行AI评审，最后由king审批 |  | Project Owner |
|  funding  | 循环 | 提交一个请款申请，最后由king审批 |  | Project Owner |
|  close  | 循环 | 终止project，最后由king审批 |  | Project Owner |

### Group合约

* Group数据组织涉及到的PDA账号

* Group合约外部请求的方法列表，使用lottery方式创建的管理组
  
|  合约方法   | 分类  | 功能描述  | 参数说明  | 签名人  |
|  ----  | ----  | ---- | ---- | ---- |
|  init  | 管理 | 初始化Group系统，建立必要的账号 | null | King |
|  launch  | 管理 | 系统开始去中心化运行，无法再进行随意添加 |  | King |
|  add  | 循环 | 添加1个Group |  | Anyone |
|  rule  | 循环 | 设置Group的签署条件，设置后即不可修改 |  | Group Owner |
|  member  | 循环 | 向Group添加1个memeber |  | Group Owner |
|  approve  | 循环 | Memeber签名验证加入Group |  | Anyone |

### Treasure合约

* Treasure数据组织涉及到的PDA账号
  
|  账户名   | 是否King  | Seeds  | 功能说明  | 数据结构  |
|  ----  | ----  | ---- | ---- | ---- |
|  treasure_setting  | ✅ | ["PROJECT_SETTING"] | 配置运行的参数，例如各种条件的百分比 | {holder:"ACCOUNT"} |

* Treasure合约外部请求的方法列表

|  合约方法   | 分类  | 功能描述  | 参数说明  | 签名人  |
|  ----  | ----  | ---- | ---- | ---- |
|  init  | 管理 | 初始化Treasure系统，建立必要的账号 | null | King |
|  config  | 管理 | 配置Treasure运行的参数，例如各种条件的百分比 | JSON | King |
|  update  | 管理 | 修改Treasure的配置 | JSON | King |
|  launch  | 管理 | 系统开始去中心化运行，无法再进行随意添加 |  | King |
|  transfer  | 循环 | 将project的token转移到国库|   | King |
|  donate  | 循环 | 向国库进行捐赠的操作 |  | Anyone |
|  pay  | 循环 | 从国库支取的操作 |  | Contract |
|  claim  | 循环 | 用户申请当期的分红 | (index,world,x,y) | Anyone |

### AI合约

* AI数据组织涉及到的PDA账号

* AI合约外部请求的方法列表
  
|  合约方法   | 分类  | 功能描述  | 参数说明  | 签名人  |
|  ----  | ----  | ---- | ---- | ---- |
|  init  | 管理 | 初始化Treasure系统，建立必要的账号 | null | King |
|  config  | 管理 | 配置Treasure运行的参数，例如各种条件的百分比 | JSON | King |
|  update  | 管理 | 修改Treasure的配置 | JSON | King |
|  launch  | 管理 | 系统开始去中心化运行，无法再进行随意添加 |  | King |
|  apply  | 循环 | 申请成为审核AI |  | AI Owner |
|  vertify  | 循环 | 验证部署好后的AI |  | Anyone |
|  claim  | 循环 | 申请审核AI的费用 |  | AI Owner |
|  judge  | 循环 | 对Project进行审议 | {project:0} | Anyone |

### Token合约

* Token数据组织涉及到的PDA账号
  
* Token合约外部请求的方法列表

|  合约方法   | 分类  | 功能描述  | 参数说明  | 签名人  |
|  ----  | ----  | ---- | ---- | ---- |
|  init  | 管理 | 初始化Treasure系统，建立必要的账号 | null | King |
|  config  | 管理 | 配置Treasure运行的参数，例如各种条件的百分比 | JSON | King |
|  update  | 管理 | 修改Treasure的配置 | JSON | King |
|  launch  | 管理 | 系统开始去中心化运行，无法再进行随意添加 |  | King |
|  create  | 循环 | 创建一个新的token，需要King的审核 | (project_id) | King |
|  transfer  | 循环 | 发送token給指定账号，需要King的审核 |  | King |
|  lock  | 循环 | 锁定剩余的 | | King |

### World合约

* World数据组织涉及到的PDA账号

|  账户名   | 是否King  | Seeds  | 功能说明  | 数据结构  |
|  ----  | ----  | ---- | ---- | ---- |
|  world_common  | ✅ | ["WORLD_COMMON"] | 所有世界的通用数据 | u8[] |
|  world_index  | ❌ | ["WORLD_INDEX"] | 记录当前正在使用的世界 | u32 |
|  world_setting  | ❌ | ["WORLD_SETTING",world_index] | 当个世界的配置 | struct |
|  auction_pool  | ❌ | ["AUCTION_POOL",world_index,round] | 拍卖池的记录 |  account[] |
|  auction_round  | ❌ | ["AUCTION_ROUND",world_index] | 拍卖轮次记录 | u32 |
|  lottery_pool | ❌ | ["LOTTERY_POOL",world_index] | 乐透池子的记录 | account[] |
|  lottery_approve | ❌ | ["LOTTERY_APPROVE",world_index] | 多次验证的hash中间记录账号 |  hash[] |
|  world_sold | ❌ | ["WORLD_SOLD",world_index] | 记录World的block被初始化的量 |  u32 |
|  world_status | ❌ | ["WORLD_STATUS",world_index,block_y] | 用来记录Block的销售状态 |  u8[512] |

* World合约外部请求的方法列表
  
|  合约方法   | 分类  | 功能描述  | 参数说明  | 签名人  |
|  ----  | ----  | ---- | ---- | ---- |
|  init  | 初始化 | 开始Meta Septopus，#0 World进入拍卖状态 |  | King |
|  auction_pool  | world拍卖 | 加入荷兰式拍卖的参与池子 |  | Anyone |
|  auction_dutch_try  | world拍卖 | 进行荷兰式拍卖的操作 |  | Account in pool |
|  auction_refund  | world拍卖 | 退回参加拍卖的押金 |  | Account in pool |
|  lottery_pool  | world拍卖 | 加入乐透式选择的池子 |  | Anyone |
|  lottery_approve  | world拍卖 | 验证hash的过程，可以产生World Owner |  | Anyone |
|  adjunct_add  | world管理 | 增加支持的adjunct |  | World owner |
|  adjunct_remove  | world管理 | 删除支持的adjunct |  | World owner |
|  world_block_price  | world管理 | 销售率达到60%后，可以对block的初始化价格进行修改 |  | World owner |
|  world_update  | world管理 | 对World的参数进行配置 |  | World owner |
|  check_in  | world管理 | 打卡登记时间 |  | World owner |
|  relieve  | world管理 | 申请移除World Owner |  | Anyone/King |
|  sell  | world循环 | 将world所有权做价销售 |  | World owner |
|  revoke  | world循环 | 撤回world所有权销售状态 |  | World owner |
|  buy  | world循环 | 购买world所有权 |  | Anyone |

### Block合约

* Block采用cNFT作为发行方式，每个block就是一个cNFT。

* Block数据组织涉及到的PDA账号

|  账户名   | 是否King  | Seeds  | 功能说明  | 数据结构  |
|  ----  | ----  | ---- | ---- | ---- |
|  block_data  | ❌ | ["BLOCK_DATA",world_index,x,y] | 单个block数据 | [elevation,status,adjunct,game_setting] |
|  block_selling  | ❌ | ["BLOCK_SELLING_LIST"] | 正在销售的block | {world,x,y,price,target}[] |
|  complain_data  | ❌ | ["COMPLAIN_DATA",world_index,x,y] | 具体的举报数据 | complain[] |
|  restore_data  | ❌ | ["RESTORE_DATA",world_index,x,y] | 申请恢复的内容 | restore object |

* Block合约外部请求的方法列表

|  合约方法   | 分类  | 功能描述  | 参数说明  | 签名人  |
|  ----  | ----  | ---- | ---- | ---- |
|  init  | 管理 | 初始化Block系统，建立必要的账号 | null | King |
|  config  | 管理 | 配置Block运行的参数，例如各种条件的百分比 | JSON | King |
|  update  | 管理 | 修改Block的配置 | JSON | King |
|  launch  | 管理 | 系统开始去中心化运行，无法再进行随意添加 |  | King |
|  own  | 管理 | 初始化占有1个block |  | Anyone |
|  abandon  | 更新 | 放弃block |  | Block Owner |
|  occupy  | 更新 | 占有废弃的block |  | Block Owner |
|  update  | 更新 | 修改block的数据 |  | Block Owner |
|  sell  | 交易 | 将block设置为销售状态 |  | Block Owner |
|  buy  | 交易 | 购买一个销售的block |  | Anyone |
|  revoke  | 交易 | 撤回block的销售状态 |  | Block Owner |
|  complain  | 管理 | 举报block的数据 |  | Anyone |
|  ban  | 管理 | 禁止显示block的内容 |  | World Owner |
|  restore  | 管理 | 申请恢复显示block的内容 |  | Block Owner |
|  recover  | 管理 | 恢复显示block的内容 |  | World Owner |

### Resource合约

* Resource数据组织涉及到的PDA账号

|  账户名   | 是否King  | Seeds  | 功能说明  | 数据结构  |
|  ----  | ----  | ---- | ---- | ---- |
|  resource_index  | ❌ | ["RESOURCE_INDEX"] | 资源重量的计数器 | u64 |
|  resource_data  | ❌ | ["RESOURCE",resource_index] | 单个资源的数据 | [] |
|  complain_index  | ❌ | ["COMPLAIN_INDEX"] | 举报数据的计数器 | u64 |
|  complain_data  | ❌ | ["COMPLAIN_DATA",complain_index] | 具体的举报数据 | [index,type,memo] |

* Resource合约外部请求的方法列表

|  合约方法   | 分类  | 功能描述  | 参数说明  | 签名人  |
|  ----  | ----  | ---- | ---- | ---- |
|  init  | 管理 | 初始化Resource系统，建立必要的账号 | null | King |
|  config  | 管理 | 配置Resource运行的参数，例如各种条件的百分比 | JSON | King |
|  update  | 管理 | 修改Resource的配置 | JSON | King |
|  launch  | 管理 | 系统开始去中心化运行，无法再进行随意添加 |  | King |
|  add  | 使用 | 添加一个资源 |  | Anyone |
|  complain  | 违规处理 | 举报一个资源 |  | Anyone |
|  ban  | 违规处理 | 禁止指定的资源 |  | Committee Multisign |
|  committee_add  | 管理 | 增加一个管理者 |  | King |
|  committee_remove  | 管理 | 增加一个管理者 |  | King |

### Adjunct合约

* Adjunct数据组织涉及到的PDA账号

|  账户名   | 是否King  | Seeds  | 功能说明  | 数据结构  |
|  ----  | ----  | ---- | ---- | ---- |
|  adjunct_index  | ❌ | ["ADJUNCT_INDEX"] | adjunct的序列号 | u32 |
|  adjunct_data | ❌ | ["ADJUNCT_DATA",index] | adjunct的code等 | u32 |
  
* Adjunct合约外部请求的方法列表

|  合约方法   | 分类  | 功能描述  | 参数说明  | 签名人  |
|  ----  | ----  | ---- | ---- | ---- |
|  init  | 管理 | 初始化Adjunct系统，建立必要的账号 | null | King |
|  config  | 管理 | 配置Adjunct运行的参数，例如各种条件的百分比 | JSON | King |
|  update  | 管理 | 修改Adjunct的配置 | JSON | King |
|  launch  | 管理 | 系统开始去中心化运行，无法再进行随意添加 |  | King |
|  add  | 使用 | 添加一个Adjunct | adjunct{} | Anyone |
|  upgrade  | 使用 | 更新Adjunct的code | adjunct{} | Anyone |
|  interpreter  | 管理 | 增加一个解释器，即非adjunct的部分| adjunct{} | Anyone |
|  update  | 管理 | 更新解释器 | adjunct{} | Anyone |

## 执行流程

### Rules Center

* Rules的执行流程列表如下：
  
|  流程名称   | 实现功能  | 涉及调用方法  |
|  ----  | ----  | ---- |
|  系统初始化   | 启动Rulse的系统，并配置好，之后进入正常去中心化运行  | Rules.init(),Rules.config(),Rules.launch()  |
|  增加rule条目   |  增加rule条目来讨论 | Rules.add(),Rules.comment() |

### King Center

### AI Center

### Meta Septopus