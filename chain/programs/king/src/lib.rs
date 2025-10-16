use anchor_lang::prelude::*;
use std::str::FromStr;

declare_id!("7tUr1JZECqmPAHqew3sjrzmygXsxCfzWoqfXaLsn6AZF");


#[program]
pub mod king {
    use super::*;

    pub fn start(ctx: Context<Start>, caller_program: Pubkey, caller: Pubkey) -> Result<()> {
        msg!(
            "✅ King.start() called from {:?}", caller_program
        );
        const key_str:&str = "3ve9oVE4P7NyiS93HGjjAoDaTuW9qearUty5ZnbfW8pM";
        let expected_entry_program: Pubkey = Pubkey::from_str(key_str).unwrap();
        require_keys_eq!(
            caller_program,
            expected_entry_program,
            KingError::UnauthorizedCaller
        );

        let king_data = &mut ctx.accounts.king_data;
        king_data.call_count += 1;
        king_data.last_caller = caller;

        msg!(
            "✅ King.start() called by Entry program: {} (total calls: {})",
            caller_program,
            king_data.call_count
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Start<'info> {
    #[account(mut)]
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

#[error_code]
pub enum KingError {
    #[msg("❌ Unauthorized caller. Only Entry program may call this.")]
    UnauthorizedCaller,
}
