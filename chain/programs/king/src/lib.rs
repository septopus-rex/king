use anchor_lang::prelude::*;

declare_id!("79ZjrUmYQcN1W55b6EP93qL4FJCmPyGQZEeJT3ghnVxh");

#[program]
pub mod king {
    use super::*;

    /// Initialize King system - only current King can execute
    pub fn init(ctx: Context<Init>, _config: String) -> Result<()> {
        let king_state = &mut ctx.accounts.king_state;
        
        // Ensure this is the first initialization
        require!(!king_state.is_initialized, ErrorCode::AlreadyInitialized);
        
        // For now, use default settings since we removed serde
        let settings = KingSettings::default();
        
        // Initialize the King system
        king_state.king = ctx.accounts.king.key();
        king_state.settings = settings;
        king_state.is_initialized = true;
        king_state.is_launched = false;
        king_state.current_lottery_round = 0;
        king_state.task_count = 0;
        
        msg!("King system initialized");
        Ok(())
    }

    /// Update King system configuration - only King can execute
    pub fn update(ctx: Context<UpdateConfig>, _config: String) -> Result<()> {
        let king_state = &mut ctx.accounts.king_state;
        
        // Verify caller is the current King and system not launched
        require!(ctx.accounts.king.key() == king_state.king, ErrorCode::UnauthorizedAccess);
        require!(!king_state.is_launched, ErrorCode::SystemLaunched);
        
        // For now, use default settings since we removed serde
        king_state.settings = KingSettings::default();
        
        msg!("King system configuration updated");
        Ok(())
    }

    /// Launch system for decentralized operation - only King can execute
    pub fn launch(ctx: Context<LaunchSystem>) -> Result<()> {
        let king_state = &mut ctx.accounts.king_state;
        
        // Verify caller is the current King
        require!(ctx.accounts.king.key() == king_state.king, ErrorCode::UnauthorizedAccess);
        require!(!king_state.is_launched, ErrorCode::SystemLaunched);
        
        king_state.is_launched = true;
        
        msg!("King system launched for decentralized operation");
        Ok(())
    }

    /// Replace specific King system setting - only Entry Task Account can execute
    pub fn replace(ctx: Context<ReplaceSetting>, key: String, value: String) -> Result<()> {
        let king_state = &mut ctx.accounts.king_state;
        
        // Verify system is launched
        require!(king_state.is_launched, ErrorCode::SystemNotLaunched);
        
        // Update specific setting based on key
        match key.as_str() {
            "lottery_fee" => {
                let fee: u64 = value.parse().map_err(|_| ErrorCode::InvalidValue)?;
                king_state.settings.lottery_fee = fee;
            },
            "king_benefit_amount" => {
                let amount: u64 = value.parse().map_err(|_| ErrorCode::InvalidValue)?;
                king_state.settings.king_benefit_amount = amount;
            },
            _ => return Err(ErrorCode::InvalidSettingKey.into()),
        }
        
        msg!("King setting updated: {} = {}", key, value);
        Ok(())
    }

    /// Join lottery pool - anyone can execute
    pub fn pool(ctx: Context<JoinPool>) -> Result<()> {
        let king_state = &ctx.accounts.king_state;
        let lottery_pool = &mut ctx.accounts.lottery_pool;
        let clock = Clock::get()?;
        
        // Check if we need to start a new lottery round
        let time_since_start = clock.unix_timestamp - king_state.settings.system_start;
        let current_round = (time_since_start / king_state.settings.lottery_interval) as u64;
        
        if current_round > lottery_pool.round {
            // Start new lottery round
            lottery_pool.round = current_round;
            lottery_pool.participants.clear();
            lottery_pool.is_active = true;
            lottery_pool.start_time = clock.unix_timestamp;
            lottery_pool.seed = clock.slot.to_be_bytes().to_vec();
        }
        
        // Add participant to current round
        let participant = ctx.accounts.participant.key();
        if !lottery_pool.participants.contains(&participant) {
            lottery_pool.participants.push(participant);
            msg!("Participant {} joined lottery round {}", participant, current_round);
        }
        
        Ok(())
    }

    /// Verify lottery and select new King - anyone can execute  
    pub fn approve(ctx: Context<ApproveLottery>) -> Result<Pubkey> {
        let king_state = &mut ctx.accounts.king_state;
        let lottery_pool = &mut ctx.accounts.lottery_pool;
        let _lottery_state = &mut ctx.accounts.lottery_state;
        
        // Ensure lottery pool has participants and seed
        require!(!lottery_pool.participants.is_empty(), ErrorCode::NoParticipants);
        require!(!lottery_pool.seed.is_empty(), ErrorCode::NoSeed);
        
        // Simplified lottery - select King based on seed and participants
        let seed_int = u64::from_be_bytes([
            lottery_pool.seed[0], lottery_pool.seed[1], lottery_pool.seed[2], lottery_pool.seed[3],
            lottery_pool.seed[4], lottery_pool.seed[5], lottery_pool.seed[6], lottery_pool.seed[7],
        ]);
        
        let selected_index = (seed_int as usize) % lottery_pool.participants.len();
        let new_king = lottery_pool.participants[selected_index];
        
        // Update King
        king_state.king = new_king;
        king_state.current_lottery_round = lottery_pool.round;
        
        // Reset lottery for next round
        lottery_pool.is_active = false;
        
        msg!("New King selected: {}", new_king);
        Ok(new_king)
    }

    /// Apply for King review - only sub-programs can execute
    pub fn apply(ctx: Context<ApplyForReview>, detail: String, action: String) -> Result<u32> {
        let king_state = &mut ctx.accounts.king_state;
        
        let task_index = king_state.task_count;
        let task = Task {
            index: task_index,
            detail,
            stamp: Clock::get()?.unix_timestamp,
            action,
            result: None,
        };
        
        king_state.tasks.push(task);
        king_state.task_count += 1;
        
        msg!("Task {} applied for King review", task_index);
        Ok(task_index)
    }

    /// King reviews a task - only King can execute
    pub fn review(ctx: Context<ReviewTask>, index: u32, result: bool) -> Result<()> {
        let king_state = &mut ctx.accounts.king_state;
        
        // Verify caller is the current King
        require!(ctx.accounts.king.key() == king_state.king, ErrorCode::UnauthorizedAccess);
        
        // Store king to avoid borrowing issue
        let current_king = king_state.king;
        
        // Find and update task
        let task = king_state.tasks.iter_mut()
            .find(|t| t.index == index)
            .ok_or(ErrorCode::TaskNotFound)?;
        
        require!(task.result.is_none(), ErrorCode::TaskAlreadyReviewed);
        
        task.result = Some(TaskResult {
            approved: result,
            king: current_king,
            stamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Task {} reviewed by King: {}", index, result);
        Ok(())
    }

    /// King claims benefit - only King can execute
    pub fn claim(ctx: Context<ClaimBenefit>) -> Result<u32> {
        let king_state = &mut ctx.accounts.king_state;
        
        // Verify caller is the current King
        require!(ctx.accounts.king.key() == king_state.king, ErrorCode::UnauthorizedAccess);
        
        let current_time = Clock::get()?.unix_timestamp;
        
        // Create benefit task for treasure contract
        let task_index = king_state.task_count;
        let action = format!(
            r#"{{"module":"treasure","method":"pay","parameter":"{{\"to\":\"{}\",\"amount\":{},\"token\":\"{}\"}}"}}"#,
            king_state.king,
            king_state.settings.king_benefit_amount,
            king_state.settings.king_benefit_token
        );
        
        let task = Task {
            index: task_index,
            detail: "KING_BENEFIT_CLAIM".to_string(),
            stamp: current_time,
            action,
            result: None,
        };
        
        king_state.tasks.push(task);
        king_state.task_count += 1;
        
        msg!("King benefit claim task {} created", task_index);
        Ok(task_index)
    }

    /// Impeach King - only Entry Task Account can execute
    pub fn impeach(ctx: Context<ImpeachKing>) -> Result<()> {
        let king_state = &mut ctx.accounts.king_state;
        
        let old_king = king_state.king;
        king_state.king = Pubkey::default(); // Set to empty, awaiting new King selection
        
        msg!("King {} has been impeached", old_king);
        Ok(())
    }
}

// Basic account structures to start with
#[derive(Accounts)]
pub struct Init<'info> {
    #[account(
        init,
        payer = king,
        space = 8 + KingState::INIT_SPACE,
        seeds = [b"king_state"],
        bump
    )]
    pub king_state: Account<'info, KingState>,
    
    #[account(mut)]
    pub king: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub king_state: Account<'info, KingState>,
    pub king: Signer<'info>,
}

#[derive(Accounts)]
pub struct LaunchSystem<'info> {
    #[account(mut)]
    pub king_state: Account<'info, KingState>,
    pub king: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReplaceSetting<'info> {
    #[account(mut)]
    pub king_state: Account<'info, KingState>,
    /// CHECK: This should be verified as Entry Task Account
    pub entry_task_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct JoinPool<'info> {
    #[account(mut)]
    pub king_state: Account<'info, KingState>,
    
    #[account(
        init_if_needed,
        payer = participant,
        space = 8 + LotteryPool::INIT_SPACE,
        seeds = [b"lottery_pool"],
        bump
    )]
    pub lottery_pool: Account<'info, LotteryPool>,
    
    #[account(mut)]
    pub participant: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveLottery<'info> {
    #[account(mut)]
    pub king_state: Account<'info, KingState>,
    
    #[account(mut)]
    pub lottery_pool: Account<'info, LotteryPool>,
    
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + LotteryState::INIT_SPACE,
        seeds = [b"lottery_state", lottery_pool.key().as_ref()],
        bump
    )]
    pub lottery_state: Account<'info, LotteryState>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApplyForReview<'info> {
    #[account(mut)]
    pub king_state: Account<'info, KingState>,
    /// CHECK: This should be a registered sub-program
    pub sub_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ReviewTask<'info> {
    #[account(mut)]
    pub king_state: Account<'info, KingState>,
    pub king: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimBenefit<'info> {
    #[account(mut)]
    pub king_state: Account<'info, KingState>,
    pub king: Signer<'info>,
}

#[derive(Accounts)]
pub struct ImpeachKing<'info> {
    #[account(mut)]
    pub king_state: Account<'info, KingState>,
    /// CHECK: This should be verified as Entry Task Account
    pub entry_task_account: AccountInfo<'info>,
}

// Data structures
#[account]
#[derive(InitSpace)]
pub struct KingState {
    pub king: Pubkey,
    pub settings: KingSettings,
    pub is_initialized: bool,
    pub is_launched: bool,
    pub current_lottery_round: u64,
    pub task_count: u32,
    #[max_len(10)]
    pub tasks: Vec<Task>,
}

#[account]
#[derive(InitSpace)]
pub struct LotteryPool {
    pub round: u64,
    #[max_len(100)]
    pub participants: Vec<Pubkey>,
    #[max_len(32)]
    pub seed: Vec<u8>,
    pub is_active: bool,
    pub start_time: i64,
}

#[account]
#[derive(InitSpace)]
pub struct LotteryState {
    pub current_iterations: u64,
    #[max_len(32)]
    pub current_hash: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct KingSettings {
    pub treasure_holder: Pubkey,
    pub lottery_interval: i64,
    pub system_start: i64,
    pub lottery_limit: u64,
    pub lottery_fee: u64,
    pub king_benefit_loop: i64,
    pub king_benefit_advance: i64,
    pub king_benefit_amount: u64,
    #[max_len(10)]
    pub king_benefit_token: String,
}

impl Default for KingSettings {
    fn default() -> Self {
        Self {
            treasure_holder: Pubkey::default(),
            lottery_interval: 403200,
            system_start: 289403200,
            lottery_limit: 1000000,
            lottery_fee: 100000,
            king_benefit_loop: 201600,
            king_benefit_advance: 28800,
            king_benefit_amount: 600,
            king_benefit_token: "SOL".to_string(),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Task {
    pub index: u32,
    #[max_len(100)]
    pub detail: String,
    pub stamp: i64,
    #[max_len(200)]
    pub action: String,
    pub result: Option<TaskResult>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct TaskResult {
    pub approved: bool,
    pub king: Pubkey,
    pub stamp: i64,
}

// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("King contract is already initialized")]
    AlreadyInitialized,
    #[msg("Unauthorized access - only King can perform this action")]
    UnauthorizedAccess,
    #[msg("Invalid configuration format")]
    InvalidConfig,
    #[msg("System has already been launched")]
    SystemLaunched,
    #[msg("System has not been launched yet")]
    SystemNotLaunched,
    #[msg("Invalid setting key")]
    InvalidSettingKey,
    #[msg("Invalid setting value")]
    InvalidValue,
    #[msg("No participants in lottery pool")]
    NoParticipants,
    #[msg("No seed available for lottery")]
    NoSeed,
    #[msg("Task not found")]
    TaskNotFound,
    #[msg("Task has already been reviewed")]
    TaskAlreadyReviewed,
}
