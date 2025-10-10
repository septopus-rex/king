use anchor_lang::prelude::*;
use std::str::FromStr; // 用于 Pubkey::from_str
use anchor_lang::AnchorSerialize;
use anchor_lang::AnchorDeserialize;
//use crate::AnchorSerialize;
use crate::Pubkey;

// --- 模拟 King 合约的结构和 ID ---
// 实际开发中，这些将通过 Cargo.toml 依赖和 Anchor IDL 导入
mod sept_king {
    // 替换为您的 King Program ID (假设的 ID)
    anchor_lang::declare_id!("3ve9oVE4P7NyiS93HGjjAoDaTuW9qearUty5ZnbfW8pM");
    
    // 模拟 King::Start Context 的账户列表
    #[derive(Clone, AnchorSerialize, AnchorDeserialize)]
    pub struct Start {
        pub entry_authority:Pubkey,
        pub treasury_state: Pubkey,
        pub user: Pubkey,
        pub system_program: Pubkey,
    }
    
    // 模拟 King 合约的指令数据
    pub fn instruction_start() -> Vec<u8> {
        // 假设 start 指令没有参数，其 Discriminator 是前 8 字节
        let mut data = [0u8; 8].to_vec();
        // 这里的 8 字节通常是 instruction discriminator，但此处简化为全 0
        data
    }
}

// 替换为您的 Entry Program ID (假设的 ID)
declare_id!("3ve9oVE4P7NyiS93HGjjAoDaTuW9qearUty5ZnbfW8pM"); 

// 定义 Entry 授权 PDA 的种子：不可更改的核心标识符
const ENTRY_SIGNER_SEEDS: &[u8] = b"entry_signer";

// King 合约的 Program ID 引用，用于 CPI 目标
pub const KING_PROGRAM_ID: Pubkey = sept_king::ID; 

#[program]
pub mod entry {
    use super::*;

    /// 入口方法：由外部用户调用，用于初始化 Septopus 状态。
    /// 核心是创建 EntryAuthority PDA，并使用其签名转发 CPI 到 King 合约。
    pub fn init_septopus(ctx: Context<InitEntry>) -> Result<()> {
        
        // 1. 初始化 Treasury 状态，并记录 Entry Authority PDA 地址
        let treasury = &mut ctx.accounts.treasury_state;
        treasury.is_initialized = true;
        // Treasury 账户中记录了谁是其授权者（即 Entry Authority PDA）
        treasury.entry_authority = ctx.accounts.entry_authority.key();
        msg!("Entry: Treasury State initialized. Authority: {}", treasury.entry_authority);


        // --- 核心安全步骤：使用 PDA 签名执行 CPI ---
        
        // 2. 准备 Entry Authority PDA 的签名种子
        let seeds = &[
            ENTRY_SIGNER_SEEDS, 
            &[ctx.bumps.entry_authority]
        ];
        // 必须是 &[&[u8]] 类型
        let signer_seeds = &[&seeds[..]];

        // 3. 构建 CPI Instruction (指令构建)
        // 获取账户元数据列表 (AccountMetas)
        let accounts: Vec<AccountMeta> = sept_king::Start {
            entry_authority: ctx.accounts.entry_authority.key(),
            treasury_state: ctx.accounts.treasury_state.key(),
            user: ctx.accounts.signer.key(),
            system_program: ctx.accounts.system_program.key(),
        }
        .to_account_metas(None); // None 表示所有账户都不是签名的

        // 获取指令数据
        let instruction_data = sept_king::instruction_start();

        // 4. 执行 CPI (Execution)
        // 创建 CPI Instruction
        let instruction = anchor_lang::solana_program::instruction::Instruction {
            program_id: KING_PROGRAM_ID,
            accounts,
            data: instruction_data,
        };
        
        // 使用 invoke_signed 执行 CPI，并传入 PDA 签名种子
        anchor_lang::solana_program::program::invoke_signed(
            &instruction,
            // 传入所有账户的 AccountInfo 列表 (重要！)
            &[
                ctx.accounts.entry_authority.to_account_info(),
                ctx.accounts.treasury_state.to_account_info(),
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.king_program.to_account_info(), // 必须包含程序 ID
            ],
            signer_seeds, // 传入 PDA 签名种子
        )?;


        msg!("Entry: CPI to King::start completed successfully.");
        Ok(())
    }
    
}

// ----------------------------------------------------
// Context 结构 (InitEntry)
// ----------------------------------------------------

#[derive(Accounts)]
pub struct InitEntry<'info> {
    /// 原始交易签名者 (外部用户)
    #[account(mut)]
    pub signer: Signer<'info>,

    /// Treasury 状态账户：存储核心配置，所有权将被 King 合约控制
    #[account(
        init, 
        payer = signer, 
        space = 8 + TreasuryState::LEN,
    )]
    pub treasury_state: Account<'info, TreasuryState>,

    /// Entry Authority PDA: 授权签名账户
    /// 这个账户的地址是 Septopus 系统内部信任的“公章”
    #[account(
        init,
        payer = signer,
        space = 8, 
        // 使用 ENTRY_SIGNER_SEEDS 派生地址，保证唯一且只有本程序能签名
        seeds = [ENTRY_SIGNER_SEEDS],
        bump,
    )]
    /// CHECK: 这是一个 Entry 合约的授权 PDA，其地址和签名由本程序保证，无需数据结构检查。
    pub entry_authority: Account<'info, Pubkey>,

    /// King Program ID: CPI 目标合约，确保 Program ID 正确
    #[account(address = KING_PROGRAM_ID)] 
    pub king_program: AccountInfo<'info>,

    /// 系统程序
    pub system_program: Program<'info, System>,
}

/// 共享状态账户，由 Entry 合约初始化，并记录 EntryAuthority 的地址
#[account]
pub struct TreasuryState {
    pub is_initialized: bool,
    pub entry_authority: Pubkey, // 记录授权 PDA 的地址，供 King 合约 cross-check
}

impl TreasuryState {
    pub const LEN: usize = 1 + 32; // bool + Pubkey
}
