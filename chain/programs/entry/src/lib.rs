#![allow(unexpected_cfgs)]  //solve the #[program] warning issue

use anchor_lang::prelude::*;

declare_id!("3ve9oVE4P7NyiS93HGjjAoDaTuW9qearUty5ZnbfW8pM");

declare_program!(pool);  // automatically generate module using program idl found in `chain/idls`
// use lever::accounts::PowerStatus;
// use lever::cpi::accounts::SwitchPower;
// use lever::cpi::switch_power;
// use lever::program::Lever;

#[program]
pub mod entry {
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
    pub payer: Signer<'info>,
}