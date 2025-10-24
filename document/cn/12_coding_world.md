# World合约设计文档

## I. 概述 (Overview)
- **合约名称**：World
- **目标链**：Solana
- **开发框架**：Anchor

### 1. 目的
World合约负责管理Septopus元宇宙中的世界创建、配置、拍卖和销售等功能，支持荷兰式拍卖和乐透式选择两种世界所有权分配机制。

### 2. 核心功能
- 世界初始化与配置管理
- 荷兰式拍卖和乐透式拍卖的世界所有权分配
- World Block的销售状态管理
- Adjunct（附加功能）支持管理
- 世界所有权交易功能

## II. 架构与账户 (Architecture & Accounts)
World合约涉及多个PDA账户来管理不同世界的数据和状态。

| 账户名称         | 类型 | 存储内容                         | 权限/签名者         |
|------------------|------|----------------------------------|---------------------|
| world_common     | PDA  | 所有世界的通用数据              | 由合约程序拥有      |
| world_index      | PDA  | 记录当前正在使用的世界          | 由合约程序拥有      |
| world_setting    | PDA  | 单个世界的配置参数              | 由合约程序拥有      |
| auction_pool     | PDA  | 拍卖池的参与记录                | 由合约程序拥有      |
| auction_round    | PDA  | 拍卖轮次记录                    | 由合约程序拥有      |
| lottery_pool     | PDA  | 乐透池子的记录                  | 由合约程序拥有      |
| lottery_approve  | PDA  | 多次验证的hash中间记录          | 由合约程序拥有      |
| world_sold       | PDA  | 记录World的block被初始化的数量 | 由合约程序拥有      |
| world_status     | PDA  | 记录Block的销售状态            | 由合约程序拥有      |

## III. 数据结构定义 (Data Structures)

### 1. world_common 账户
存储所有世界的通用配置数据。

| 字段名称         | 类型 (Rust) | 说明                  |
|------------------|-------------|-----------------------|
| -------     | -------       | -------      |

### 2. world_setting 账户
存储单个世界的详细配置参数。

| 字段名称                | 类型 (Rust) | 说明                                |
|-------------------------|-------------|-------------------------------------|
| -------     | -------       | -------      |

### 3. auction_pool 账户
记录拍卖参与者的信息。

| 字段名称         | 类型 (Rust) | 说明                                |
|------------------|-------------|-------------------------------------|
| -------     | -------       | -------      |

### 4. lottery_pool 账户
记录乐透式选择的参与者信息。

| 字段名称         | 类型 (Rust) | 说明                                |
|------------------|-------------|-------------------------------------|
| -------     | -------       | -------      |

### 5. world_status 账户
记录World中各个Block的销售状态。

| 字段名称         | 类型 (Rust) | 说明                                |
|------------------|-------------|-------------------------------------|
| -------     | -------       | -------      |

## IV. 指令与逻辑 (Instructions & Logic)

### 1. init
功能：初始化Meta Septopus，启动#0 World进入拍卖状态。

| 账户 (Inputs)      | 约束/权限        | 备注                 |
|---------------------|-------------------|----------------------|
| world_common        | Init, Payer       | 世界通用数据账户     |
| world_index         | Init, Payer       | 世界索引账户         |
| signer              | Signer            | King地址             |

### 2. auction_pool
功能：加入荷兰式拍卖的参与池子。

| 账户 (Inputs)      | 约束/权限            | 备注                 |
|---------------------|-----------------------|----------------------|
| auction_pool        | Mut                   | 拍卖池账户           |
| participant         | Signer, Writable      | 参与者地址           |

### 3. auction_dutch_try
功能：进行荷兰式拍卖的操作。

| 账户 (Inputs)      | 约束/权限            | 备注                 |
|---------------------|-----------------------|----------------------|
| auction_pool        | Mut                   | 拍卖池账户           |
| world_setting       | Mut                   | 世界配置账户         |
| participant         | Signer                | 池中参与者          |

### 4. lottery_pool
功能：加入乐透式选择的池子。

| 账户 (Inputs)      | 约束/权限            | 备注                 |
|---------------------|-----------------------|----------------------|
| lottery_pool        | Mut                   | 乐透池账户           |
| participant         | Signer                | 参与者地址           |

### 5. adjunct_add
功能：增加支持的adjunct。

| 账户 (Inputs)      | 约束/权限            | 备注                 |
|---------------------|-----------------------|----------------------|
| world_setting       | Mut                   | 世界配置账户         |
| adjunct_data        | Ref                   | Adjunct数据账户      |
| world_owner         | Signer                | 世界所有者           |

### 6. world_block_price
功能：修改block的初始化价格（销售率达到60%后）。

| 账户 (Inputs)      | 约束/权限            | 备注                 |
|---------------------|-----------------------|----------------------|
| world_setting       | Mut                   | 世界配置账户         |
| world_status        | Ref                   | 世界状态账户         |
| world_owner         | Signer                | 世界所有者           |

## V. 错误代码 (Error Codes)

| 错误代码 (Rust Enum)      | 说明 (Chinese)           |
|-----------------------------|--------------------------|
| InvalidWorldIndex           | 无效的世界索引           |
| AuctionNotActive            | 拍卖未激活               |
| InsufficientDeposit         | 押金不足                 |
| NotInAuctionPool            | 不在拍卖池中             |
| SaleThresholdNotReached     | 销售率未达到阈值         |
| InvalidAdjunct              | 无效的Adjunct            |
| NotWorldOwner               | 非世界所有者             |
| WorldAlreadySold            | 世界已售出               |

该设计完整覆盖了World合约的所有功能模块，包括拍卖管理、配置管理、状态跟踪等核心功能。已根据要求去除world_common账户中的max_worlds和active_worlds字段。
