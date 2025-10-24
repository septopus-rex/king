# Resource合约设计文档

## I. 概述 (Overview)

**合约名称**: Resource
**目标链**: Solana
**开发框架**: Anchor

### 1. 目的

本合约旨在为 Septopus 生态系统提供一个安全、可扩展的资源管理机制，负责游戏资源（如 3D 模型、纹理、角色等）的链上注册和审核管理，资源文件实际存储在 IPFS 上，合约只存储文件哈希和元数据。

### 2. 核心功能

- **资源注册管理**: 资源的 IPFS 哈希注册、审核和禁用
- **举报机制**: 用户可举报违规资源，由委员会多签处理
- **委员会管理**: King 可管理资源审核委员会成员
- **资源协议支持**: 支持多种资源类型（module、texture、avatar 等）的标准格式

---

## II. 架构与账户 (Architecture & Accounts)

本合约涉及四个主要的链上 PDA 账户，用于管理资源注册信息和举报信息。

| **账户名称**     | **类型** | **存储内容**                | **权限/签名者** |
|-------------------|----------|-----------------------------|-----------------|
| resource_index    | PDA      | 资源数量的计数器            | 由合约程序拥有  |
| resource_data     | PDA      | 单个资源的注册信息（IPFS 哈希等） | 由合约程序拥有  |
| complain_index    | PDA      | 举报数量的计数器            | 由合约程序拥有  |
| complain_data     | PDA      | 具体的举报详情              | 由合约程序拥有  |

---

## III. 数据结构定义 (Data Structures)

### 1. **ResourceIndex** 账户
存储资源总数和索引信息。

| **字段名称**       | **类型 (Rust)** | **说明**            |
|--------------------|-----------------|---------------------|
| total_count        | u64             | 资源总数            |
| last_index         | u64             | 最后创建的资源索引  |

### 2. **ResourceData** 账户
存储单个资源的链上注册信息。

| **字段名称**       | **类型 (Rust)** | **说明**                            |
|--------------------|-----------------|-------------------------------------|
| index              | u64             | 资源唯一索引                        |
| owner              | Pubkey          | 资源创建者地址                      |
| resource_type      | String          | 资源类型（module/texture/avatar 等） |
| ipfs_hash          | String          | IPFS 文件哈希（CID）                |
| file_size          | u64             | 文件大小（字节）                    |
| metadata_hash      | String          | 元数据哈希                          |
| is_banned          | bool            | 是否被禁用                          |
| create_timestamp   | i64             | 创建时间戳                          |

### 3. **ComplainIndex** 账户
存储举报统计信息。

| **字段名称**       | **类型 (Rust)** | **说明**            |
|--------------------|-----------------|---------------------|
| total_complains    | u64             | 举报总数            |
| pending_complains  | u64             | 待处理举报数        |

### 4. **ComplainData** 账户
存储单个举报详情。

| **字段名称**       | **类型 (Rust)** | **说明**                            |
|--------------------|-----------------|-------------------------------------|
| complain_id        | u64             | 举报唯一 ID                         |
| resource_index     | u64             | 被举报资源索引                      |
| complainant        | Pubkey          | 举报人地址                          |
| complain_type      | String          | 举报类型                            |
| memo               | String          | 举报说明                            |
| status             | ComplainStatus  | 举报状态（pending/processed）       |
| timestamp          | i64             | 举报时间戳                          |

---

## IV. 指令与逻辑 (Instructions & Logic)

所有指令必须进行严格的权限检查。

### 1. **init**
功能：初始化 Resource 系统，建立必要的 PDA 账户。

#### 账户 (Inputs)

| **账户**          | **约束/权限**  | **备注**               |
|-------------------|---------------|-----------------------|
| resource_index    | Init, Payer   | 资源索引 PDA          |
| complain_index    | Init, Payer   | 举报索引 PDA          |
| king              | Signer        | 必须为 King 地址      |

#### 逻辑步骤

1. 验证签名者为 King 权限
2. 初始化 `resource_index` 和 `complain_index` 账户

---

### 2. **add**
功能：注册一个新的资源（在 IPFS 上存储后注册哈希）。

#### 账户 (Inputs)

| **账户**          | **约束/权限**  | **备注**               |
|-------------------|---------------|-----------------------|
| resource_index    | Mut           | 资源索引 PDA          |
| resource_data     | Init, Payer   | 新资源数据 PDA        |
| signer            | Signer        | 资源创建者            |
| system_program    | Ref           | 系统程序              |

#### 逻辑步骤

1. 从 `resource_index` 获取新资源索引
2. 验证 IPFS 哈希格式有效性
3. 创建新的 `resource_data` 账户存储哈希信息
4. 更新 `resource_index` 计数器

---

### 3. **complain**
功能：举报违规资源。

#### 账户 (Inputs)

| **账户**          | **约束/权限**  | **备注**               |
|-------------------|---------------|-----------------------|
| complain_index    | Mut           | 举报索引 PDA          |
| complain_data     | Init, Payer   | 新举报数据 PDA        |
| resource_data     | Ref           | 被举报资源数据        |
| signer            | Signer        | 举报人                |

#### 逻辑步骤

1. 验证资源存在且未被禁用
2. 创建 `complain_data` 账户记录举报信息
3. 更新 `complain_index` 计数器

---

### 4. **ban**
功能：委员会多签禁用违规资源。

#### 账户 (Inputs)

| **账户**          | **约束/权限**  | **备注**               |
|-------------------|---------------|-----------------------|
| resource_data     | Mut           | 目标资源数据          |
| committee_multisig | Signer       | 委员会多签地址        |
| complain_data     | Ref           | 相关举报数据          |

#### 逻辑步骤

1. 验证委员会多签权限
2. 验证举报状态为 `pending`
3. 设置 `resource_data.is_banned = true`
4. 更新举报状态为 `processed`

---

### 5. **committee_add**
功能：King 添加委员会成员。

#### 账户 (Inputs)

| **账户**          | **约束/权限**  | **备注**               |
|-------------------|---------------|-----------------------|
| committee_config  | Mut           | 委员会配置 PDA        |
| king              | Signer        | 必须为 King 地址      |
| new_member        | Ref           | 新成员地址            |

#### 逻辑步骤

1. 验证 King 权限
2. 检查新成员是否已存在
3. 添加新成员到委员会列表

---

### 6. **committee_remove**
功能：King 移除委员会成员。

#### 账户 (Inputs)

| **账户**          | **约束/权限**  | **备注**               |
|-------------------|---------------|-----------------------|
| committee_config  | Mut           | 委员会配置 PDA        |
| king              | Signer        | 必须为 King 地址      |
| remove_member     | Ref           | 要移除的成员地址      |

#### 逻辑步骤

1. 验证 King 权限
2. 验证成员存在
3. 从委员会列表中移除成员

---

## V. 错误代码 (Error Codes)

| **错误代码 (Rust Enum)**        | **说明 (Chinese)**         |
|---------------------------------|----------------------------|
| InvalidKingSignature            | 操作者不是有效的 King 地址  |
| ResourceAlreadyBanned           | 资源已被禁用              |
| InvalidIPFSHash                 | 无效的 IPFS 哈希格式       |
| ComplainNotPending              | 举报不是待处理状态         |
| CommitteeMemberExists           | 委员会成员已存在          |
| CommitteeMemberNotFound         | 委员会成员不存在          |
| InvalidCommitteeMultisig        | 无效的委员会多签地址       |

---

## VI. 资源协议规范 (Resource Protocol)

合约管理的资源遵循以下IPFS存储格式：

### IPFS文件结构
```json
{
  "type": "resource_type",
  "format": "file_format",
  "raw": "BASE64_ENCODE_STRING",
  "metadata": {
    // 类型特定的元数据
  }
}

### 链上存储内容

- **IPFS 哈希（CID）**: 指向上述 JSON 文件的哈希
- **文件大小**: 用于验证完整性
- **元数据哈希**: `metadata` 字段的哈希值
- **资源类型**: 用于分类检索

### 支持的类型

- module
- texture
- avatar
- lines
- block
- adjunct
- chord
