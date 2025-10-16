# King Center功能说明

## 随机算法

### King随机

* King设置强密码`PASSWORD`，使用AES加密64位的随机盐`RANDOM_SALT`得到`AES_ENCRIED_CODE`，将其记录在Solana的第`n`块上。
* 在n+200之后，King通过合约公开密码`PASSWORD`。
* 通过以下算法，计算出随机的64位的哈希。种子为`${RANDOM_SALT}_${SOLANA_BLOCK_HASH[n]}_${SOLANA_BLOCK_HASH[n+100]}_${SOLANA_BLOCK_HASH[n+101]}`。
* 对种子循环SHA256计算100万次，得到最终的随机哈希。

* 拟放弃上面的方法，使用更简单的PoW的循环计算方式，可以快速的更换King，来降低被劫持的可能性。

### 参与者随机

* 放弃以下的方法，使用PoW的方式，分批计算来实现。

---

* 从`选举池`里，随机选择7位参与者，用密码`Password`来加密随机盐`RANDOM_SALT`得到`ENCRIED_TEXT`保存到链上。
* 7位参与者都写链成功后，`King`执行在第n块执行合约，确认开始。
* 通过以下算法，计算出随机的64位的哈希。种子为`${RANDOM_SALT}_${SOLANA_BLOCK_HASH[n]}_${SOLANA_BLOCK_HASH[n+100]}_${SOLANA_BLOCK_HASH[n+101]}`。
* 对种子循环SHA256计算100万次，得到最终的随机哈希。

## 打卡及供养

* 每日签名打卡，确保King是活跃的。
* 领取每周的收入

## 签署功能

* 签署Rule的审批
* 签署Project的审批

## 移交功能

### 新王登基

* 设置新的King账号
* 签署加密短文上链（测试不同的合约可以正常运行）
* 账号打款功能测试
* 签署新王登基的协议（测试不同的合约可以正常运行）
* 基本信息设置，如社交账号、Github等

### 旧王交接期

* 领取每周的收入