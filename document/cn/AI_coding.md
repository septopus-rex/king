# Septopus合约开发说明

## **I. 概述 (Overview)**

合约名称： Septopus System  
目标链： Solana  
开发框架： Anchor
合约组织方式： CPI

### **1. 目的**

本项目旨在为 **Septopus** 生态系统提供一个灵活、安全的合约框架。它实现了一个**Entry**合约，作为主入口，管理诸多的**子合约**，实现**Septopus**的管理目标，来实现以下的合约请求方式。

1. **需要King审批的操作:** 通过**Entry**进行请求，验证请求是否来自于**King**。
2. **需要验证请求来自Entry的操作:** 通过**Entry**进行请求，附带验证的PDA账号供子账号进行验证请求来源。
3. **不需要King审批的操作:** 可以通过构造参数的方式，直接请求到子合约。

--------

## Entry合约功能说明

### 子合约管理

* 合约功能如下表

|  合约方法   | 分类  | 功能描述  | 参数说明  | 签名人  |
|  :----  | :----  | :---- | :---- | :---- |
|  add  | 增加一个子合约 | 添加子合约的Program ID |  | King |
|  remove  | 移除一个子合约 | 移除子合约的Program ID |  | King |

### 子合约请求

## 子合约功能说明

* 对于不需要"king"验证的请求，可以构造直接来对子合约进行访问。

### 生命周期管理

* `init`:
* `config`:
* `launch`:
* `update`:

### 合约独立功能

#### World合约

* 合约功能如下表

|  账户名   | 是否King  | Seeds  | 功能说明  | 数据结构  |
|  :----  | :----  | :---- | :---- | :---- |
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
|  :----  | :----  | :---- | :---- | :---- |
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
|  sell  | world循环 | 将world所有权做价销售 |  | World owner |
|  revoke  | world循环 | 撤回world所有权销售状态 |  | World owner |
|  buy  | world循环 | 购买world所有权 |  | Anyone |

#### Block合约

* 合约PDA账号如下表

|  账户名   | 是否King  | Seeds  | 功能说明  | 数据结构  |
|  :----  | :----  | :---- | :---- | :---- |
|  block_data  | ❌ | ["BLOCK_DATA",world_index,x,y] | 单个block数据 | [elevation,status,adjunct,game_setting] |
|  block_selling  | ❌ | ["BLOCK_SELLING_LIST"] | 正在销售的block | {world,x,y,price,target}[] |
|  complain_data  | ❌ | ["COMPLAIN_DATA",world_index,x,y] | 具体的举报数据 | complain[] |
|  restore_data  | ❌ | ["RESTORE_DATA",world_index,x,y] | 申请恢复的内容 | restore object |

* 合约功能如下表

|  合约方法   | 分类  | 功能描述  | 参数说明  | 签名人  |
|  :----  | :----  | :---- | :---- | :---- |
|  own  | 更新 | 初始化占有1个block |  | Anyone |
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

## 开发要求

* 生成完整的单元测试
* 有说明文档和部署文档
* 优化合约的Gas费用