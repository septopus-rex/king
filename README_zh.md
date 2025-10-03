# King管理中心

## 合约入口

* 使用Solana的CPI结构来组织`Septopus`的所有合约，使用单一入口。

## 合约结构

* `Septopus`的功能，合约拆分情况如下：

|  合约名称   | 功能描述  | 详情  |
|  ----  | ----  | ----  |
|  Rules  | Rules的数据、Rules的讨论、Rules的修改 |  |
|  King  |  King的乐透选取、King的日常签到、King的审批签署、King的支付审核 |  |
|  Project  |  Project的立项、 |  |
|  Token  |  项目token的管理（创建、分发、锁定） |  |
|  AI  |  AI的审核、AI的部署 |  |
|  Adjunct  |  Adjunct创建、Adjunct更新等 |  |
|  Resoure  |  Resouce创建、Resouce更新、Resouce举报 |  |
|  World  |  World的拍卖、World的配置、World的销售状态 |  |
|  Block  |  数据保存、Block交易、Block举报、Block禁显、Block申请恢复、Block恢复 |  |

* ❌ ✅
  
### King合约

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
|  launch  | 管理 | 系统开始去中心化运行，无法再进行随意添加 |  | King |
|  add  | 循环 | 添加1条rules |  | King/Anyone |
|  abandon  | 循环 | 废弃投票通过的rule |  | Anyone |
|  start  | 循环 | 发起对新的rule进行投票 |  | Anyone |
|  vote  | 循环 | 针对rule进行投票的动作 |  | Anyone |
|  comment  | 循环 | 对任何一条rule进行评论 |  | Anyone |
|  accept  | 循环 | 接受一条新的rule |  | Anyone |

### Project合约

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

### Block合约

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
|  init  | 循环 | 初始化1个block |  | Anyone |
|  abandon  | 循环 | 放弃block |  | Block Owner |
|  occupy  | 循环 | 占有废弃的block |  | Block Owner |
|  sell  | 交易 | 将block设置为销售状态 |  | Block Owner |
|  buy  | 交易 | 购买一个销售的block |  | Anyone |
|  revoke  | 交易 | 撤回block的销售状态 |  | Block Owner |
|  update  | 更新 | 修改block的数据 |  | Block Owner |
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
|  add  | 使用 | 添加一个Adjunct | adjunct{} | Anyone |