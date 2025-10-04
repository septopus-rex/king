#![allow(unexpected_cfgs)]  //solve the #[program] warning issue

use anchor_lang::prelude::*;

declare_id!("3ve9oVE4P7NyiS93HGjjAoDaTuW9qearUty5ZnbfW8pM");


#[program]
pub mod pool {
    use super::*;

    pub fn router(
        ctx: Context<Sample>,
        index: u32,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Sample<'info> {
    #[account(mut)]
    pub payer: Signer<'info>
}