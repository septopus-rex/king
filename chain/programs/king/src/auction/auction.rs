use {
    anchor_lang::prelude::*,
    anchor_lang::system_program,
    //std::str::FromStr,
    //serde_json::{Value},
};
use crate::Pubkey;

use crate::constants::{
    SOLANA_PDA_LEN,
    KING_COUNTER,
    KingIndex,
};

pub fn init(
    ctx: Context<InitSystem>,      //default from system
) -> Result<()> {
    
    //check wether from `entry program` 
    //let _index = &mut ctx.accounts.king_index;

    if ctx.accounts.treasury_state.entry_authority != ctx.accounts.entry_authority.key() {
            // 理论上这个错误不应该发生，但作为双重保障
            return Err(ErrorCode::InvalidAuthorityMapping.into());
        }


    Ok(())
}

#[derive(Accounts)]
pub struct InitSystem<'info> {
    pub entry_authority: AccountInfo<'info>, 

    #[account(
        // 确保 Treasury 账户由 King 合约拥有
        mut, 
        // 关键检查：确保 TreasuryState.entry_authority == entry_authority 账户
        has_one = entry_authority
    )]
    pub treasury_state: Account<'info, TreasuryState>,
    
    // 原始用户签名者
    pub user: AccountInfo<'info>,

    // 系统程序
    pub system_program: Program<'info, System>,
    
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        space = SOLANA_PDA_LEN + KingIndex::INIT_SPACE,     
        payer = payer,
        seeds = [KING_COUNTER],
        bump,
    )]
    pub king_index: Account<'info, KingIndex>,
}


#[account]
pub struct TreasuryState {
    pub is_initialized: bool,
    pub entry_authority: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The authority mapping in the treasury state is invalid.")]
    InvalidAuthorityMapping,
}