# **King 智能合约设计文档**

## **I. 概述 (Overview)**

合约名称：King
程序 ID（示例）：`7tUr1JZECqmPAHqew3sjrzmygXsxCfzWoqfXaLsn6AZF`
开发框架：Anchor
所属系统：Septopus 去中心化模块系统
依赖：EntryRouter（主入口）

---

### **1. 合约定位**

`King` 是整个系统的**治理与审批核心合约**，
用于负责以下三类功能：

1. **生命周期管理**：负责子系统初始化、配置、启动、更新；
2. **去中心化选王机制（Lottery）**：通过开放的 SHA256 随机挑战产生新的 King；
3. **治理与审批（Governance）**：负责处理系统内的申请、审批、费用管理等。

所有关键操作（如 config、launch、update）都需 King 审批后才能执行。

---

### **2. 与 EntryRouter 的关系**

| 方向              | 描述                                  |
| :-------------- | :---------------------------------- |
| Entry → King    | Entry 发起审批请求，由 King 验证请求来源。         |
| King → Entry    | 通过 PDA 机制，验证调用确实来自 Entry。           |
| Frontend → King | 允许直接发起公开类操作（lottery、pool、impeach等）。 |

---

## **II. 架构与账户 (Architecture & Accounts)**

### **1. 模块结构**

| 模块         | 描述                    |
| :--------- | :-------------------- |
| Lifecycle  | 负责 King 合约本身及系统生命周期管理 |
| Lottery    | 控制 King 的选举流程         |
| Governance | 管理审批、申请、费用与放弃等治理类行为   |

---

### **2. 主要账户**

| 账户名称          | 类型          | 说明                              |
| :------------ | :---------- | :------------------------------ |
| KingData      | **Account** | 存储 King 合约核心状态（当前 King、状态、参数等）。 |
| PoolData      | **Account** | 存储乐透参与者信息及资金池。                  |
| ConfigData    | **Account** | 存储系统配置参数。                       |
| Treasury      | **Account** | 系统金库，用于费用申请与分发。                 |
| VerifyPDA     | **PDA**     | 用于验证请求是否来自 EntryRouter。         |
| Payer         | **Signer**  | 用户钱包（用于支付操作）。                   |
| SystemProgram | **Program** | Solana 系统程序引用。                  |

---

## **III. 数据结构定义 (Data Structures)**

### **1. KingData**

| 字段            | 类型     | 说明                                      |
| :------------ | :----- | :-------------------------------------- |
| current_king  | Pubkey | 当前 King 的地址                             |
| last_king     | Pubkey | 上一任 King 的地址                            |
| election_time | i64    | 上次选举时间戳                                 |
| total_rounds  | u64    | 总选举轮次                                   |
| state         | u8     | 系统状态（0=Init,1=Config,2=Launch,3=Update） |
| treasury      | Pubkey | 系统金库账户                                  |
| config        | Pubkey | 配置数据账户                                  |

---

### **2. ConfigData**

| 字段                    | 类型  | 说明                    |
| :-------------------- | :-- | :-------------------- |
| min_pool_sol          | u64 | 加入池的最小 SOL 数（默认 0.01） |
| approve_iterations    | u32 | SHA256 批次数量（默认10）     |
| approve_per_iteration | u32 | 每次计算次数（默认10000）       |
| next_lottery_interval | i64 | 下次选举的时间间隔             |
| impeach_threshold     | u64 | 弹劾条件参数                |

---

### **3. PoolData**

| 字段           | 类型          | 说明                 |
| :----------- | :---------- | :----------------- |
| participants | Vec<Pubkey> | 当前轮次参与者            |
| total_pool   | u64         | 累积资金池（单位 lamports） |
| round        | u64         | 当前轮次编号             |

---

## **IV. 指令与逻辑 (Instructions & Logic)**

---

### **1. init**

**功能**
初始化 King 合约，建立核心账户并存储初始状态。

| 账户             | 说明           |
| :------------- | :----------- |
| payer          | 管理员 signer   |
| king_data      | 新建账户，存储合约主状态 |
| system_program | 系统程序         |

**逻辑**

1. 创建 `KingData`；
2. 初始化 state=0 (Init)，并记录当前调用者为初始 King；
3. 分配 Treasury 和 ConfigData。

---

### **2. config**

**功能**
配置系统参数，仅在 Init 阶段允许修改。
此操作必须经 King 审批。

| 调用来源        | 需要 King 审批 |
| :---------- | :--------- |
| EntryRouter | ✅ 是        |
| King 自身     | ✅ 是        |

**逻辑**

1. 验证调用来源（`caller_program == EntryRouter.program_id` 或当前 King）。
2. 修改 ConfigData 参数。
3. 设置状态为 Config(1)。

---

### **3. launch**

**功能**
启动系统，使合约进入“去中心化运行”阶段（不可再直接修改配置）。
需要 King 审批。

| 状态   | 条件      |
| :--- | :------ |
| 允许状态 | Config  |
| 执行权限 | 当前 King |

**逻辑**

1. 校验当前状态为 Config。
2. 验证调用来源为 King 或 EntryRouter。
3. 修改 state = Launch(2)。

---

### **4. update**

**功能**
在去中心化运行后（state=Launch），允许通过 King 审批方式更新系统配置。

| 调用来源        | 条件  |
| :---------- | :-- |
| EntryRouter | ✅ 是 |
| King 直接调用   | ✅ 是 |

---

### **5. lottery**

**功能**
发起一轮新的 King 选举。任何人可调用。

| 调用来源     | 条件                                  |
| :------- | :---------------------------------- |
| Frontend | ✅ 任何人                               |
| 限制       | 间隔时间 > config.next_lottery_interval |

**逻辑**

1. 检查当前时间是否符合选举间隔；
2. 清空上轮池并重置状态；
3. state=Lottery。

---

### **6. pool**

**功能**
加入选举池。任何人都可加入并支付0.01SOL。

| 调用来源     | 条件              |
| :------- | :-------------- |
| Frontend | ✅ 任何人           |
| 限制       | 付款金额 ≥ 0.01 SOL |

**逻辑**

1. 收取 lamports；
2. 将参与者 Pubkey 加入 PoolData.participants；
3. 更新 total_pool。

---

### **7. approve**

**功能**
确认选举结果，执行随机哈希选取新 King。
任何人可执行，但需消耗计算资源。

| 调用来源     | 条件                      |
| :------- | :---------------------- |
| Frontend | ✅ 任何人                   |
| 限制       | 逐步执行，共10轮，每轮计算1万次SHA256 |

**逻辑**

1. 执行分段哈希计算（防止单次计算超出CU）；
2. 最终结果用于决定新的 King。
3. 更新 KingData.current_king。

---

### **8. impeach**

**功能**
弹劾当前 King，触发下一轮选举。

| 调用来源     | 条件                      |
| :------- | :---------------------- |
| Frontend | ✅ 任何人                   |
| 限制       | 满足 impeach_threshold 条件 |

---

### **9. apply**

**功能**
发起需要 King 审批的申请。任何人可请求。

| 调用来源     | 条件                            |
| :------- | :---------------------------- |
| Frontend | ✅ 任何人                         |
| 备注       | 由 EntryRouter 转发至 King.review |

---

### **10. review**

**功能**
King 审批外部请求。

| 调用来源        | 条件       |
| :---------- | :------- |
| King signer | ✅ 仅 King |
| EntryRouter | ✅ 转发请求   |

---

### **11. claim**

**功能**
King 提取金库费用。

| 调用来源        | 条件       |
| :---------- | :------- |
| King signer | ✅ 仅 King |
| 限制          | 有足够余额    |

---

### **12. abandon**

**功能**
King 主动放弃王位，触发新的选举轮次。

| 调用来源        | 条件       |
| :---------- | :------- |
| King signer | ✅ 仅 King |

---

## **V. 安全机制 (Security Model)**

| 项目          | 描述                                         |
| :---------- | :----------------------------------------- |
| Entry 验证    | 子合约内通过 `verify_pda` 验证调用来源确实为 EntryRouter。 |
| King 审批验证   | 所有系统级操作均需验证 `caller == current_king`。      |
| Lottery 公平性 | 使用多轮 SHA256 运算作为随机过程，结果不可预测。               |
| 去中心化保障      | 一旦进入 Launch 状态，不再允许直接更改配置，只能通过 King 审批。    |
| 金库安全        | 所有转账操作仅限 King 或 Entry 授权的 CPI。             |

---

## **VI. 状态流转 (State Machine)**

```
┌──────────┐
│  Init    │
└────┬─────┘
     │ config()
     ▼
┌──────────┐
│  Config  │
└────┬─────┘
     │ launch()
     ▼
┌──────────┐
│  Launch  │
└────┬─────┘
     │ update()
     ▼
┌──────────┐
│  Update  │
└──────────┘
```

---

## **VII. 错误码 (Error Codes)**

| 错误代码              | 描述                          |
| :---------------- | :-------------------------- |
| Unauthorized      | 非 King 或 EntryRouter 调用。    |
| InvalidState      | 当前状态不支持该操作。                 |
| InsufficientFunds | 加入池时支付金额不足。                 |
| LotteryTooSoon    | 当前时间未到下次选举周期。               |
| ApprovePending    | 哈希计算尚未完成。                   |
| TreasuryEmpty     | 金库余额不足。                     |
| InvalidPDA        | verify_pda 不匹配 EntryRouter。 |

---

## **VIII. 与 EntryRouter 的交互**

| 操作 | Entry → King 调用 | King 侧验证                                      |
| :----------------------- | :-------------- | :-------------------------------------------- |
| config / launch / update | ✅ 由 Entry 发起    | 验证 `caller_program == EntryRouter.program_id` |
| 审批请求（apply/review）       | ✅ 由 Entry 发起    | 通过 verify_pda 校验                              |
| lottery / pool / impeach | ❌ 前端直接调用        | 不做来源限制                                        |

---

## ✅ **IX. 总结**

`King` 是系统治理的核心合约，
负责掌控整个生态的状态流转、参数调整、选举与资金流通。
配合 `EntryRouter` 可以实现：

* **权限分层治理（Entry → King → Submodules）**
* **开放参与机制（lottery + pool）**
* **可追踪、可验证的调用授权（verify_pda）**
