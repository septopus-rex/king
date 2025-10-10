#![allow(unexpected_cfgs)]  //solve the #[program] warning issue

use anchor_lang::prelude::*;

declare_id!("3ve9oVE4P7NyiS93HGjjAoDaTuW9qearUty5ZnbfW8pM");

// automatically generate module using program idl found in `chain/idls`
declare_program!(pool);
use pool::accounts::*;
use pool::cpi::start;
use pool::cpi::accounts::Start;
use pool::program::*;

const ENTRY_SIGNER_SEEDS: &[u8] = b"entry_signer";

#[program]
pub mod entry {
    use super::*;

    pub fn router(
        ctx: Context<Sample>,
        index: u32,
    ) -> Result<()> {

        let treasury = &mut ctx.accounts.treasury_state;
        treasury.is_initialized = true;
        treasury.entry_authority = ctx.accounts.entry_authority.key();
        msg!("Entry: Treasury State initialized. Authority: {}", treasury.entry_authority);

        let cpi_ctx = CpiContext::new(
            ctx.accounts.pool_program.to_account_info(),
            Start {
                entry_authority: ctx.accounts.entry_authority.to_account_info(),
                treasury_state: ctx.accounts.treasury_state.to_account_info(),
                user:ctx.accounts.payer.to_account_info(),
                payer:ctx.accounts.payer.to_account_info(),
                system_program:ctx.accounts.system_program.to_account_info(),
            },
        );
        start(cpi_ctx)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Sample<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init, 
        payer = payer, 
        space = 8 + TreasuryState::LEN,
    )]
    pub treasury_state: Account<'info, TreasuryState>,

     #[account(
        mut,
        seeds = [ENTRY_SIGNER_SEEDS],
        bump,
    )]
    pub entry_authority: SystemAccount<'info>,

    pub pool_program: Program<'info, Pool>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct TreasuryState {
    pub is_initialized: bool,
    pub entry_authority: Pubkey, // 记录授权 PDA 的地址，供 King 合约 cross-check
}

impl TreasuryState {
    pub const LEN: usize = 1 + 32; // bool + Pubkey
}
