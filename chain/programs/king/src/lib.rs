#![allow(unexpected_cfgs)]  //solve the #[program] warning issue

use anchor_lang::prelude::*;
use crate::Pubkey;
use std::str::FromStr;

declare_id!("7tUr1JZECqmPAHqew3sjrzmygXsxCfzWoqfXaLsn6AZF");

#[program]
pub mod pool {
    use super::*;

    //1. CPI test and Entry PDA authority test.
    pub fn start(
        ctx: Context<StartKing>,
    ) -> Result<()> {

         if ctx.accounts.treasury_state.entry_authority != ctx.accounts.entry_authority.key() {
            // 理论上这个错误不应该发生，但作为双重保障
            return Err(ErrorCode::InvalidAuthorityMapping.into());
        }

        msg!("King: SUCCESSFULLY VERIFIED ENTRY AUTHORITY.");
        msg!("King: Septopus核心初始化逻辑开始...");
        
        // 此处的逻辑只有在 Entry 合约用其 PDA 签名后才能执行。
        // ... 例如：初始化 King 的资金池，或分配初始治理权 ...
        //pub const ENTRY_PROGRAM_ID: Pubkey =  Pubkey::from_str("3ve9oVE4P7NyiS93HGjjAoDaTuW9qearUty5ZnbfW8pM").unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartKing<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: `Entry` PDA account, check authoritt
    pub entry_authority: AccountInfo<'info>, 

    /// Treasury 状态：检查这个账户是否由正确的授权者控制
    #[account(
        // 确保 Treasury 账户由 King 合约拥有
        mut, 
        // 关键检查：确保 TreasuryState.entry_authority == entry_authority 账户
        has_one = entry_authority
    )]
    pub treasury_state: Account<'info, TreasuryState>,

    /// CHECK: signer from `Entry`
    pub user: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}


// ----------------------------------------------------
// 账户结构 (TreasuryState - 必须与 Entry 合约中的定义一致)
// ----------------------------------------------------

#[account]
pub struct TreasuryState {
    pub is_initialized: bool,
    pub entry_authority: Pubkey,
}

// ----------------------------------------------------
// 错误代码
// ----------------------------------------------------

#[error_code]
pub enum ErrorCode {
    #[msg("The authority mapping in the treasury state is invalid.")]
    InvalidAuthorityMapping,
}
