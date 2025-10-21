# **智能合约设计文档：分级多签金库（Tiered Multisig Vault）**

## **I. 概述 (Overview)**

合约名称： TieredMultisigVault  
目标链： Solana  
开发框架： Anchor

### **1. 目的**

本项目旨在为 **Septopus** 生态系统提供一个灵活、安全的资金管理机制。它实现了一个**混合治理模型**，用于控制一个资产金库（Vault PDA），确保资金转移指令可以通过以下两种条件之一执行：

1. **特权通过 (Designated Owner Override):** 指定的单个特权地址签名批准。  
2. **多数通过 (Remaining Majority):** 剩余的授权地址达到预设的多数阈值 $M'$。

### **2. 核心功能**

* 创建多签金库，并设置特权地址和剩余多数阈值。  
* 创建、批准和执行任意类型的 CPI（Cross-Program Instruction）交易提案。  
* 使用 Program Derived Address (PDA) 作为金库签名者，持有资产。

## **II. 架构与账户 (Architecture & Accounts)**

本合约涉及三个主要的链上账户，其中两个为可变状态账户，一个为资产金库。

| 账户名称 | 类型 | 存储内容 | 权限/签名者 |
| :---- | :---- | :---- | :---- |
| MultisigState | **Account (State)** | 多签规则（Owners, Threshold, Bumps）。 | 由合约程序拥有。 |
| VaultPDA | **PDA** | 实际持有 SOL 和 SPL Token 资产的账户。 | 由 MultisigState 账户和预设种子派生，只能由本合约签名。 |
| Transaction | **Account (State)** | 待批准的 CPI 交易提案的细节。 | 由合约程序拥有。 |

## **III. 数据结构定义 (Data Structures)**

### **1. MultisigState 账户**

该账户存储多签的配置和状态信息。

| 字段名称 | 类型 (Rust) | 说明 |
| :---- | :---- | :---- |
| vault_bump | u8 | 用于派生 VaultPDA 的 Bump 值。 |
| designated_owner | Pubkey | 拥有 **一票通过** 权限的特权地址。 |
| remaining_owners | Vec<Pubkey> | 剩余的授权签名者地址列表。 |
| remaining_threshold | u8 | 剩余签名者中，批准交易所需的最小票数 $M'$。 |
| owner_set_seqno | u32 | 所有者集合版本号，用于防止重放攻击。 |

### **2. Transaction 账户**

该账户存储一个待执行的 CPI 提案的完整细节和批准状态。

| 字段名称 | 类型 (Rust) | 说明 |
| :---- | :---- | :---- |
| multisig | Pubkey | 关联的 MultisigState 账户 Key。 |
| program_id | Pubkey | CPI 的目标程序 ID。 |
| accounts | Vec<TransactionAccountMeta> | CPI 所需的账户元数据列表。 |
| data | Vec<u8> | CPI 的指令数据。 |
| is_designated_signed | bool | 特权地址是否已签名。 |
| remaining_did_sign | Vec<bool> | 剩余签名者中，谁已签名（顺序与 remaining_owners 对应）。 |
| executed | bool | 交易是否已成功执行。 |

### **3. TransactionAccountMeta 结构**

用于序列化 CPI 所需账户的元数据。

| 字段名称 | 类型 (Rust) | 说明 |
| :---- | :---- | :---- |
| pubkey | Pubkey | 账户地址。 |
| is_signer | bool | 是否需要签名（对于 CPI 来说，通常为 False，因为签名由 PDA 提供）。 |
| is_writable | bool | 账户是否可写。 |

## **IV. 指令与逻辑 (Instructions & Logic)**

所有指令必须进行严格的权限检查。

### **1. initialize_multisig**

**功能：** 创建 MultisigState 账户并初始化 VaultPDA。

| 账户 (Inputs) | 约束/权限 | 备注 |
| :---- | :---- | :---- |
| multisig | **Init, Payer** | 待初始化的 MultisigState 账户。 |
| vault_pda | **Init, Payer, Seeds** | PDA，用于存放资产。Seeds: ["vault", multisig_key] |
| signer | **Signer, Writable** | 交易发起人/Payer。 |

**逻辑步骤：**

1. 验证 remaining_threshold 必须 $le$ remaining_owners.len()。  
2. 初始化 MultisigState，设置所有者、阈值和 vault_bump。

### **2. create_transaction**

**功能：** 提交一个新的多签交易提案。

| 账户 (Inputs) | 约束/权限 | 备注 |
| :---- | :---- | :---- |
| multisig | **Mut** | 关联的 MultisigState 账户。 |
| transaction | **Init, Payer** | 待初始化的 Transaction 账户。 |
| proposer | **Signer** | 交易发起人。**必须是 designated_owner 或 remaining_owners 之一。** |

**逻辑步骤：**

1. 验证 proposer 必须是多签的任一有效 owner。  
2. 初始化 Transaction 账户，记录 CPI 细节。  
3. **自动批准：** 将 proposer 对应的签名状态（is_designated_signed 或 remaining_did_sign）标记为 True。

### **3. approve**

**功能：** 授权 owner 签署批准提案。

| 账户 (Inputs) | 约束/权限 | 备注 |
| :---- | :---- | :---- |
| multisig | **Ref** | 关联的 MultisigState 账户。 |
| transaction | **Mut** | 待批准的 Transaction 账户。 |
| owner | **Signer** | 签署批准的 owner 地址。**必须是 designated_owner 或 remaining_owners 之一。** |

**逻辑步骤：**

1. 验证 transaction.executed 必须为 False。  
2. **如果 owner 等于 multisig.designated_owner:**  
   * 设置 transaction.is_designated_signed = True。  
3. **如果 owner 属于 multisig.remaining_owners:**  
   * 找到其索引，设置 transaction.remaining_did_sign[index] = True。  
   * 验证该 owner 之前未签名。

### **4. execute_transaction**

**功能：** 检查阈值，如果满足条件，则使用 VaultPDA 签名执行 CPI。

| 账户 (Inputs) | 约束/权限 | 备注 |
| :---- | :---- | :---- |
| multisig | **Ref** | 关联的 MultisigState 账户。 |
| vault_pda | **Ref** | 资产金库 PDA。 |
| transaction | **Mut** | 待执行的 Transaction 账户。 |
| target_program | **Ref** | CPI 目标程序。 |
| Remaining Accounts | **Context** | CPI 所需的所有账户列表（由 transaction.accounts 定义）。 |

**逻辑步骤 (核心校验)：**

1. 验证 transaction.executed 必须为 False。  
2. **检查执行条件 (逻辑分支)：**  
   * **IF** transaction.is_designated_signed is True  
   * **OR** transaction.remaining_did_sign 中 True 的数量 $ge$ multisig.remaining_threshold  
3. **如果满足条件：**  
   * 从 multisig 账户获取 vault_bump 和 seeds。  
   * 构建 CPI 指令（使用 transaction.program_id, transaction.accounts, transaction.data）。  
   * 使用 vault_pda 的 seeds 调用 solana_program::program::invoke_signed。  
   * 设置 transaction.executed = True。  
4. **如果不满足条件：** 返回 NotEnoughSignatures 错误。

## **V. 错误代码 (Error Codes)**

| 错误代码 (Rust Enum) | 说明 (Chinese) |
| :---- | :---- |
| InvalidThreshold | 剩余多数阈值 ($M'$) 设置大于剩余 owner 数量 ($N-1$)。 |
| InvalidOwner | 交易发起人或签名者不属于任何授权 owner 列表。 |
| AlreadySigned | 账户已批准过该交易。 |
| TransactionExecuted | 尝试批准或执行已完成的交易。 |
| NotEnoughSignatures | 批准票数未达到执行所需的阈值（特权或多数）。 |
| InvalidVaultBump | 提供的 Vault PDA Bump 值与存储的不匹配。 |

