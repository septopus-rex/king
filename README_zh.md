# King管理中心

## 合约入口

* 使用Solana的CPI结构来组织`Septopus`的所有合约，使用单一入口。

## 合约结构

* `Septopus`的功能，合约拆分情况如下：

|  合约名称   | 功能描述  | 详情  |
|  ----  | ----  | ----  |
|  Rules  | Rules的数据、Rules的讨论、Rules的修改 |  |
|  King  |  King的乐透选取、King的日常签到、King的审批签署、King的支付审核 |  |
|  Token  |  项目token的管理（创建、分发、锁定） |  |
|  Project  |  Project的立项、 |  |
|  AI  |  AI的审核、AI的部署 |  |
|  Adjunct  |  Adjunct创建、Adjunct更新等 |  |
|  Resoure  |  Resouce创建、Resouce更新、Resouce举报 |  |
|  World  |  World的拍卖、World的配置、World的销售状态 |  |
|  Block  |  数据保存、Block交易、Block举报、Block禁显、Block申请恢复、Block恢复 |  |

### King合约

* ❌ ✅

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
|  block_data  | ❌ | ["BLOCK_DATA",world_indee,x,y] | 单个block数据 | [elevation,status,adjunct,game_setting] |
|  block_selling  | ❌ | ["BLOCK_SELLING_LIST"] | 正在销售的block | {world,x,y,price,target}[] |

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

### Rules合约