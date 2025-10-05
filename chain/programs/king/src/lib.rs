#![allow(unexpected_cfgs)]  //solve the #[program] warning issue

use anchor_lang::prelude::*;

declare_id!("3ve9oVE4P7NyiS93HGjjAoDaTuW9qearUty5ZnbfW8pM");

use {
    auction::*,
};
pub mod auction;
pub mod constants;

#[program]
pub mod pool {
    use super::*;

    pub fn init(
        ctx: Context<InitSystem>,
    ) -> Result<()> {
        auction::init(ctx)
    }

    // pub fn approve(
    //     _ctx: Context<Sample>,
    //     _index: u32,
    // ) -> Result<()> {
    //     Ok(())
    // }
}

// #[derive(Accounts)]
// pub struct Sample<'info> {
//     #[account(mut)]
//     pub payer: Signer<'info>,
// }