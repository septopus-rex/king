# **智能合约设计文档：Entry 路由器（Entry Router）**

## **I. 概述 (Overview)**

合约名称：EntryRouter
目标链：Solana
开发框架：Anchor
程序 ID：`3ve9oVE4P7NyiS93HGjjAoDaTuW9qearUty5ZnbfW8pM`

### **1. 目的**

`EntryRouter` 是整个 **Septopus 模块化生态** 的入口调度中心（Router）。
其主要目标是实现一个统一的入口，通过 **安全、可审计的方式** 调用和转发请求到各个子合约（如 King、Builder、Market 等）。

该合约在架构上充当：

* **调用代理（Proxy）**：前端只与 EntryRouter 交互；
* **访问控制器（Gatekeeper）**：验证调用来源与子程序是否被篡改；
* **模块注册表（Module Registry）**：统一记录已注册的子合约及其版本/哈希。

---

## **II. 架构与账户 (Architecture & Accounts)**

### **1. 模块关系**

| 合约                 | 角色         | 调用方式              | 说明                            |
| :----------------- | :--------- | :---------------- | :---------------------------- |
| EntryRouter        | 调度中心       | 前端调用              | 统一入口。通过 `run()` 向目标子合约发起 CPI。 |
| King               | 子合约（具有审批权） | 由 Entry 调用        | 用于验证 Entry 请求来源合法性。           |
| Builder / Market 等 | 普通子合约      | 由 Entry 调用或前端直接调用 | 普通业务逻辑。                       |

---

### **2. 主要账户**

| 账户名称           | 类型                  | 说明                           |
| :------------- | :------------------ | :--------------------------- |
| ModuleRegistry | **Account (State)** | 存储所有子合约注册信息（ID、名称、哈希等）。      |
| VerifyPDA      | **PDA**             | 用于验证请求是否由 Entry 发起。由子合约读取验证。 |
| Caller         | **Signer**          | 发起调用的用户钱包地址。                 |
| TargetProgram  | **Program**         | 目标子合约程序。                     |
| SystemProgram  | **Program**         | 系统程序引用。                      |

---

## **III. 数据结构定义 (Data Structures)**

### **1. ModuleRegistry**

| 字段名称    | 类型 (Rust)       | 说明                      |
| :------ | :-------------- | :---------------------- |
| owner   | Pubkey          | Registry 管理员地址（通常为部署者）。 |
| modules | Vec<ModuleInfo> | 注册的所有子合约列表。             |

### **2. ModuleInfo**

| 字段名称             | 类型       | 说明                       |
| :--------------- | :------- | :----------------------- |
| name             | String   | 模块名称（如 "king"、"market"）。 |
| program_id       | Pubkey   | 模块对应的 Program ID。        |
| code_hash        | [u8; 32] | 模块字节码哈希值，用于防篡改验证。        |
| require_approval | bool     | 是否需要 King 审批才能执行。        |

### **3. VerifyPDA**

| 字段名称             | 类型     | 说明                                     |
| :--------------- | :----- | :------------------------------------- |
| last_request     | u64    | 最近一次验证请求的 slot。                        |
| authorized_entry | Pubkey | 授权的 EntryRouter Program ID（只接受该来源的调用）。 |

---

## **IV. 指令与逻辑 (Instructions & Logic)**

### **1. initialize_registry**

**功能：** 初始化模块注册表。

| 账户             | 约束          | 说明                   |
| :------------- | :---------- | :------------------- |
| owner          | Signer      | 管理员。                 |
| registry       | Init, Payer | ModuleRegistry 状态账户。 |
| system_program | Program     | 系统程序。                |

**逻辑：**

1. 创建并初始化 ModuleRegistry。
2. 设置 owner 和空的 modules 数组。

---

### **2. register_module**

**功能：** 注册一个新的子合约模块。

| 账户       | 约束     | 说明            |
| :------- | :----- | :------------ |
| owner    | Signer | Registry 所有者。 |
| registry | Mut    | 存储模块信息。       |

**输入参数：**

* `name: String`
* `program_id: Pubkey`
* `code_hash: [u8; 32]`
* `require_approval: bool`

**逻辑：**

1. 验证调用者是 registry.owner。
2. 计算并存储模块信息。
3. 确保不存在同名模块或重复 program_id。

---

### **3. run**

**功能：** 路由器统一入口。由前端调用，Entry 负责执行实际 CPI。

| 账户             | 约束      | 说明          |
| :------------- | :------ | :---------- |
| payer          | Signer  | 交易发起人。      |
| registry       | Mut     | 模块注册表。      |
| verify_pda     | PDA     | Entry 验证账户。 |
| system_program | Program | 系统程序。       |

**输入参数：**

* `target_program: Pubkey`
* `encoded_data: Vec<u8>`（由前端构造）
* `code_hash: [u8; 32]`

**逻辑：**

1. 检查目标程序是否在 registry.modules 中。
2. 验证目标 program_id 的 code_hash 是否匹配（防止被替换部署）。
3. 如果 require_approval = true：

   * 调用 King 合约的 `approve_entry()` 方法。
   * 由 King 验证请求来源为 EntryRouter。
4. 如果 require_approval = false：

   * 直接构造 CPI 并执行。
5. 调用目标程序的 `encoded_data`。

---

### **4. verify_entry_pda**

**功能：** 在子合约中验证请求是否来源于 EntryRouter。

| 账户             | 约束      | 说明                |
| :------------- | :------ | :---------------- |
| verify_pda     | Mut     | 用于验证 Entry 的 PDA。 |
| system_program | Program | 系统程序。             |

**逻辑：**

1. 验证 PDA 是否由 EntryRouter 派生。
2. 校验 `authorized_entry == EntryRouter.program_id`。
3. 如果匹配则验证通过，否则返回错误。

---

## **V. 安全机制 (Security Model)**

| 类型      | 描述                                                    |
| :------ | :---------------------------------------------------- |
| 子合约篡改防护 | 通过 `code_hash` 校验确保部署的字节码未被替换。                        |
| 调用来源验证  | King 合约验证 `caller_program == EntryRouter.program_id`。 |
| PDA 验证  | Entry 派生 `verify_pda`，由子合约验证 Entry 调用来源。              |
| 访问控制    | Registry 由 owner 管理，防止恶意注册模块。                         |

---

## **VI. 调用流程 (Call Flow)**

### **1. 需要 King 审批的操作**

```text
Frontend → Entry.run(target_program=Market, require_approval=true)
   ↳ Entry 调用 King.approve_entry()
       ↳ King 验证 caller_program == EntryRouter.program_id
   ↳ 审批通过后，Entry 执行目标模块逻辑
```

### **2. 需要验证 Entry 来源的操作**

```text
Frontend → Entry.run(target_program=Builder)
   ↳ Entry 构造 verify_pda (seed=["verify", entry_id])
   ↳ Builder::method() 读取 verify_pda，确认请求来源合法
```

### **3. 不需要 King 审批的操作**

```text
Frontend → Builder::method() 直接调用
   ↳ 无需 Entry 或 King 审批
```

### **4. Entry 验证子合约未被替换**

```text
Entry.run()
   ↳ 从 registry 获取 (program_id, code_hash)
   ↳ 对比目标部署的 code_hash
   ↳ 不匹配则拒绝执行
```

---

## **VII. 错误码 (Error Codes)**

| 错误代码                 | 说明                       |
| :------------------- | :----------------------- |
| Unauthorized         | 非注册模块或调用者无权操作。           |
| ModuleNotFound       | 目标模块未注册。                 |
| CodeHashMismatch     | 子合约部署已被篡改。               |
| KingApprovalRequired | 模块需要 King 审批但未通过验证。      |
| InvalidVerifyPDA     | verify_pda 不匹配 Entry 派生。 |