use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_program;
use std::str::FromStr;

declare_id!("3ve9oVE4P7NyiS93HGjjAoDaTuW9qearUty5ZnbfW8pM");

#[program]
pub mod entry {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        msg!("🚀 Entry.init() called");

        let king_program = ctx.accounts.king_program.key();
        let entry_program_id = ctx.program_id;
        let payer = ctx.accounts.payer.key();

        // 构造传递给 King 的调用数据
        let data = KingInstruction::Start {
            caller_program: *entry_program_id,
            caller: payer,
        }
        .try_to_vec()?;


        let accounts = vec![
            AccountMeta::new(ctx.accounts.king_data.key(), false),
            AccountMeta::new_readonly(system_program::ID, false),
        ];
        

        let ix = Instruction {
            program_id: king_program,
            accounts,
            data,
        };
        msg!("Instruction: {:?}", ix.clone());
        
        invoke_signed(&ix, &[ctx.accounts.king_data.to_account_info()], &[])?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

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

// 用于构造 King 合约调用数据
#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum KingInstruction {
    Start {
        caller_program: Pubkey,
        caller: Pubkey,
    },
}

// 定义 King 数据结构以便 Entry 知道其空间大小
#[account]
pub struct KingData {
    pub call_count: u64,
    pub last_caller: Pubkey,
}

impl KingData {
    pub const LEN: usize = 8 + 8 + 32;
}

// 定义 King Program 类型
#[derive(Clone)]
pub struct King;

impl anchor_lang::Id for King {
    fn id() -> Pubkey {
        let key_str = "7tUr1JZECqmPAHqew3sjrzmygXsxCfzWoqfXaLsn6AZF";
        Pubkey::from_str(key_str).unwrap()
    }
}
