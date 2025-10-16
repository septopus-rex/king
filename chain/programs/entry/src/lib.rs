use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke_signed,
    system_program,
};
use std::str::FromStr;

declare_id!("3ve9oVE4P7NyiS93HGjjAoDaTuW9qearUty5ZnbfW8pM");

#[program]
pub mod entry {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        msg!("🚀 Entry.init() called");

        let king_program_id = Pubkey::from_str("7tUr1JZECqmPAHqew3sjrzmygXsxCfzWoqfXaLsn6AZF").unwrap();
        let entry_program_id = ctx.program_id;
        let payer = ctx.accounts.payer.key();

        //-------------------------------------------------------------------
        // 1️⃣ 计算目标方法 "start" 的 discriminator
        //-------------------------------------------------------------------
        use anchor_lang::Discriminator;
        // discriminator = sha256("global:start")[..8]
        let discriminator = anchor_lang::solana_program::hash::hash("global:start".as_bytes());
        let discriminator_bytes = &discriminator.to_bytes()[..8];

        //-------------------------------------------------------------------
        // 2️⃣ 序列化参数数据
        //-------------------------------------------------------------------
        let mut data = Vec::with_capacity(8 + 32 + 32);
        data.extend_from_slice(discriminator_bytes); // 方法ID
        data.extend_from_slice(entry_program_id.as_ref()); // 参数 caller_program
        data.extend_from_slice(payer.as_ref()); // 参数 caller

        //-------------------------------------------------------------------
        // 3️⃣ 构造 CPI 调用目标的账户列表
        //-------------------------------------------------------------------
        let accounts = vec![
            AccountMeta::new(ctx.accounts.king_data.key(), false),
            AccountMeta::new_readonly(system_program::ID, false),
        ];

        //-------------------------------------------------------------------
        // 4️⃣ 组装 Instruction
        //-------------------------------------------------------------------
        let ix = Instruction {
            program_id: king_program_id,
            accounts,
            data,
        };

        //-------------------------------------------------------------------
        // 5️⃣ 执行 CPI 调用
        //-------------------------------------------------------------------
        invoke_signed(
            &ix,
            &[
                ctx.accounts.king_data.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[], // 无 signer seeds
        )?;

        msg!("✅ CPI call to King.start() done!");
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

    pub system_program: Program<'info, System>,
}

#[account]
pub struct KingData {
    pub call_count: u64,
    pub last_caller: Pubkey,
}

impl KingData {
    pub const LEN: usize = 8 + 8 + 32;
}