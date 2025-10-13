use anchor_lang::prelude::*;
// 1. 确保在文件顶层引入 AnchorSerialize，使 KingInstruction 可以使用 try_to_vec_with_discriminator()
use anchor_lang::AnchorSerialize; 
use anchor_lang::solana_program::instruction::{Instruction as SolanaInstruction, AccountMeta};
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_program;
use std::str::FromStr;

declare_id!("3ve9oVE4P7NyiS93HGjjAoDaTuW9qearUty5ZnbfW8pM");

#[program]
pub mod entry {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        msg!("🚀 Entry.init() called");

        let king_program_id = ctx.accounts.king_program.key();
        let entry_program_id = ctx.program_id;
        
        // --------------------------------------------------------------------------------
        // 修复 E0614 错误：不能直接对 Signer<'info> 对象的 key() 结果进行两次解引用。
        // payer.key() 返回的是 &Pubkey，它不是一个 Account<'info, Pubkey>，
        // 且它指向的是一个借用的 Pubkey。
        // 正确的做法是直接调用 key() 获取 &Pubkey，然后解引用一次。
        let payer_pubkey = ctx.accounts.payer.key(); 
        
        // --------------------------------------------------------------------------------

        let instruction_struct = KingInstruction::Start {
            caller_program: *entry_program_id,
            // 修复 caller 字段：直接解引用 payer_pubkey，得到 Pubkey 值。
            caller: payer_pubkey, 
        };

        // --------------------------------------------------------------------------------
        // 修复 E0599 错误：由于 KingInstruction 定义在 entry 模块外，
        // try_to_vec_with_discriminator 必须在作用域内。
        // 虽然在文件顶部引入，但 Rust 有时需要在模块内再次导入 Trait 或使用完整路径。
        // 确保 KingInstruction::try_to_vec_with_discriminator() 可用。
        // --------------------------------------------------------------------------------
        // let data_with_discriminator: Vec<u8> = instruction_struct
        //     .try_to_vec_with_discriminator() 
        //     .map_err(|_| ErrorCode::InstructionDidNotSerialize)?; 

        // SOL transfer instruction discriminator
        let instruction_discriminator: u32 = 2;
        let mut instruction_data = Vec::with_capacity(4 + 8);
        instruction_data.extend_from_slice(&instruction_discriminator.to_le_bytes());

        let ix_accounts = vec![
            AccountMeta::new(ctx.accounts.king_data.key(), false),
            AccountMeta::new_readonly(system_program::ID, false), 
        ];

        let ix = SolanaInstruction {
            program_id: king_program_id,
            accounts: ix_accounts,
            data: instruction_data,
        };
        
        msg!("Instruction data (with discriminator): {:?}", ix.data.clone());

        invoke_signed(
            &ix, 
            &[
                ctx.accounts.king_data.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ], 
            &[]
        )?;

        Ok(())
    }
}

// ... (Accounts 和数据结构定义保持不变)

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // ... (king_data, king_program, system_program 保持不变)
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + KingData::LEN,
        seeds = [b"king_data"],
        bump
    )]
    pub king_data: Account<'info, KingData>,

    pub king_program: Program<'info, King>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum KingInstruction {
    Start {
        caller_program: Pubkey,
        caller: Pubkey,
    },
}

#[account]
pub struct KingData {
    pub call_count: u64,
    pub last_caller: Pubkey,
}

impl KingData {
    pub const LEN: usize = 8 + 8 + 32;
}

#[derive(Clone)]
pub struct King;

impl anchor_lang::Id for King {
    fn id() -> Pubkey {
        let key_str = "7tUr1JZECqmPAHqew3sjrzmygXsxCfzWoqfXaLsn6AZF";
        Pubkey::from_str(key_str).unwrap()
    }
}