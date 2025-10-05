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
    let _index = &mut ctx.accounts.king_index;

    Ok(())
}

#[derive(Accounts)]
pub struct InitSystem<'info> {
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

    pub system_program: Program<'info, System>,
}
