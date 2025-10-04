#![allow(unexpected_cfgs)]  //solve the #[program] warning issue

use anchor_lang::prelude::*;

declare_id!("7tUr1JZECqmPAHqew3sjrzmygXsxCfzWoqfXaLsn6AZF");


#[program]
pub mod entry {
    use super::*;

    pub fn router(
        ctx: Context<BanResource>,
        index: u32,
    ) -> Result<()> {
        Ok(())
    }
}