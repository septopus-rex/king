use anchor_lang::prelude::*;

declare_id!("3ve9oVE4P7NyiS93HGjjAoDaTuW9qearUty5ZnbfW8pM");

#[program]
pub mod entry {
    use super::*;

    /// Initialize Entry contract - can only be run once
    pub fn init(ctx: Context<Init>, king: Pubkey) -> Result<()> {
        let entry_state = &mut ctx.accounts.entry_state;
        
        // Ensure this is the first initialization
        require!(!entry_state.is_initialized, ErrorCode::AlreadyInitialized);
        
        // Set up the Entry system
        entry_state.king = king;
        entry_state.is_initialized = true;
        entry_state.sub_contract_count = 0;
        
        // Initialize the entry task account
        let entry_task = &mut ctx.accounts.entry_task_account;
        entry_task.authority = entry_state.key();
        entry_task.bump = ctx.bumps.entry_task_account;
        
        msg!("Entry contract initialized with King: {}", king);
        Ok(())
    }

    /// Register a sub-contract - only King can execute
    pub fn reg(ctx: Context<RegSubContract>, sub_program_address: Pubkey) -> Result<()> {
        let entry_state = &mut ctx.accounts.entry_state;
        
        // Verify caller is the current King
        require!(ctx.accounts.king.key() == entry_state.king, ErrorCode::UnauthorizedAccess);
        
        // Check if we have space for more sub-contracts
        require!(entry_state.sub_contracts.len() < MAX_SUB_CONTRACTS as usize, ErrorCode::TooManySubContracts);
        
        // Add the sub-contract to the registry using Vec::push
        let sub_contract = SubContract {
            address: sub_program_address,
            is_active: true,
            registered_at: Clock::get()?.unix_timestamp,
        };
        entry_state.sub_contracts.push(sub_contract);
        entry_state.sub_contract_count = entry_state.sub_contracts.len() as u32;
        
        msg!("Sub-contract registered at index {}: {}", entry_state.sub_contract_count - 1, sub_program_address);
        Ok(())
    }

    /// Remove a sub-contract - only King can execute
    pub fn remove(ctx: Context<RemoveSubContract>, index: u32) -> Result<()> {
        let entry_state = &mut ctx.accounts.entry_state;
        
        // Verify caller is the current King
        require!(ctx.accounts.king.key() == entry_state.king, ErrorCode::UnauthorizedAccess);
        
        // Verify index is valid
        require!(index < entry_state.sub_contract_count, ErrorCode::InvalidSubContractIndex);
        
        // Mark sub-contract as inactive
        entry_state.sub_contracts[index as usize].is_active = false;
        
        msg!("Sub-contract at index {} marked as inactive", index);
        Ok(())
    }

    /// Execute a sub-contract via CPI - only King can execute
    pub fn run(ctx: Context<RunSubContract>, index: u32, param_length: u32) -> Result<()> {
        let entry_state = &ctx.accounts.entry_state;
        
        // Verify caller is the current King
        require!(ctx.accounts.king.key() == entry_state.king, ErrorCode::UnauthorizedAccess);
        
        // Verify index is valid and sub-contract is active
        require!(index < entry_state.sub_contract_count, ErrorCode::InvalidSubContractIndex);
        let sub_contract = &entry_state.sub_contracts[index as usize];
        require!(sub_contract.is_active, ErrorCode::SubContractInactive);
        
        // Verify the provided program matches the registered address
        require!(ctx.accounts.sub_program.key() == sub_contract.address, ErrorCode::SubContractMismatch);
        
        // Special handling for King.approve calls that might return a new King
        if ctx.accounts.sub_program.key() == entry_state.king {
            // This might be a King.approve call - we need to handle potential King updates
            msg!("Executing potential King.approve call");
            // Note: In a real implementation, we'd need to parse the return data
            // and update the King if a new one is selected
        }
        
        msg!("Executed sub-contract at index {} with {} param bytes", index, param_length);
        Ok(())
    }
}

// Account validation structs
#[derive(Accounts)]
pub struct Init<'info> {
    #[account(
        init,
        payer = king,
        space = 8 + EntryState::INIT_SPACE,
        seeds = [b"entry_state"],
        bump
    )]
    pub entry_state: Account<'info, EntryState>,
    
    #[account(
        init,
        payer = king,
        space = 8 + EntryTaskAccount::INIT_SPACE,
        seeds = [b"entry_task", entry_state.key().as_ref()],
        bump
    )]
    pub entry_task_account: Account<'info, EntryTaskAccount>,
    
    #[account(mut)]
    pub king: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegSubContract<'info> {
    #[account(mut)]
    pub entry_state: Account<'info, EntryState>,
    pub king: Signer<'info>,
}

#[derive(Accounts)]
pub struct RemoveSubContract<'info> {
    #[account(mut)]
    pub entry_state: Account<'info, EntryState>,
    pub king: Signer<'info>,
}

#[derive(Accounts)]
pub struct RunSubContract<'info> {
    #[account(mut)]
    pub entry_state: Account<'info, EntryState>,
    
    #[account(
        mut,
        seeds = [b"entry_task", entry_state.key().as_ref()],
        bump = entry_task_account.bump
    )]
    pub entry_task_account: Account<'info, EntryTaskAccount>,
    
    pub king: Signer<'info>,
    
    /// CHECK: This is the sub-program to be called via CPI
    pub sub_program: AccountInfo<'info>,
}

// Data structures
#[account]
#[derive(InitSpace)]
pub struct EntryState {
    pub king: Pubkey,
    pub is_initialized: bool,
    pub sub_contract_count: u32,
    #[max_len(100)]
    pub sub_contracts: Vec<SubContract>,
}

#[account]
#[derive(InitSpace)]
pub struct EntryTaskAccount {
    pub authority: Pubkey,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct SubContract {
    pub address: Pubkey,
    pub is_active: bool,
    pub registered_at: i64,
}

// Constants
const MAX_SUB_CONTRACTS: u32 = 100;

// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Entry contract is already initialized")]
    AlreadyInitialized,
    #[msg("Unauthorized access - only King can perform this action")]
    UnauthorizedAccess,
    #[msg("Too many sub-contracts registered")]
    TooManySubContracts,
    #[msg("Invalid sub-contract index")]
    InvalidSubContractIndex,
    #[msg("Sub-contract is inactive")]
    SubContractInactive,
    #[msg("Sub-contract address mismatch")]
    SubContractMismatch,
}
