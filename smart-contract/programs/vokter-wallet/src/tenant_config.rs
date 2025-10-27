use anchor_lang::prelude::*;

// *** TENANT_CONFIG.RS UPDATED AT 14:15 - MINT POLICY + BULLETPROOF FEES ***

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum MintPolicy {
    /// Allow all SPL tokens
    AllowAll,
    /// Only allow specific mints (stored separately)
    Allowlist,
    /// Block all SPL tokens
    BlockAll,
}

impl anchor_lang::Space for MintPolicy {
    const INIT_SPACE: usize = 1; // Enum discriminant size
}

impl Default for MintPolicy {
    fn default() -> Self {
        MintPolicy::AllowAll
    }
}

#[account]
#[derive(InitSpace)]
pub struct TenantConfig {
    pub tenant_id: Pubkey,
    pub treasury: Pubkey,
    pub admin: Pubkey,
    pub fee_basis_points: u64,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub version: u8,
    pub max_daily_transactions: u16,
    pub max_transaction_amount: u64,
    pub min_transaction_amount: u64,
    pub mint_policy: MintPolicy,
    pub _reserved: [u8; 31],
}

impl TenantConfig {
    pub const MAX_FEE_BASIS_POINTS: u64 = 500;
    pub const MIN_FEE_BASIS_POINTS: u64 = 0;
    pub const DEFAULT_MAX_DAILY_TRANSACTIONS: u16 = 500;
    pub const DEFAULT_MAX_TRANSACTION_AMOUNT: u64 = 1_000_000_000_000;
    pub const DEFAULT_MIN_TRANSACTION_AMOUNT: u64 = 50_000_000;

    pub fn initialize(
        &mut self,
        tenant_id: Pubkey,
        treasury: Pubkey,
        admin: Pubkey,
        fee_basis_points: u64,
        clock: &Clock,
    ) -> Result<()> {
        require!(
            fee_basis_points <= Self::MAX_FEE_BASIS_POINTS,
            crate::ErrorCode::InvalidFeeRate
        );
        require!(
            treasury != admin,
            crate::ErrorCode::TreasuryCannotBeAdmin
        );
        require!(
            tenant_id != Pubkey::default(),
            crate::ErrorCode::InvalidTenantId
        );

        self.tenant_id = tenant_id;
        self.treasury = treasury;
        self.admin = admin;
        self.fee_basis_points = fee_basis_points;
        self.is_active = true;
        self.created_at = clock.unix_timestamp;
        self.updated_at = clock.unix_timestamp;
        self.version = 1;
        self.max_daily_transactions = Self::DEFAULT_MAX_DAILY_TRANSACTIONS;
        self.max_transaction_amount = Self::DEFAULT_MAX_TRANSACTION_AMOUNT;
        self.min_transaction_amount = Self::DEFAULT_MIN_TRANSACTION_AMOUNT;
        self.mint_policy = MintPolicy::AllowAll;
        self._reserved = [0; 31];

        Ok(())
    }

    pub fn update_treasury(&mut self, new_treasury: Pubkey, clock: &Clock) -> Result<()> {
        require!(
            new_treasury != self.admin,
            crate::ErrorCode::TreasuryCannotBeAdmin
        );
        require!(
            new_treasury != Pubkey::default(),
            crate::ErrorCode::InvalidTreasuryAddress
        );

        self.treasury = new_treasury;
        self.updated_at = clock.unix_timestamp;
        Ok(())
    }

    pub fn update_fee_rate(&mut self, new_fee_basis_points: u64, clock: &Clock) -> Result<()> {
        require!(
            new_fee_basis_points <= Self::MAX_FEE_BASIS_POINTS,
            crate::ErrorCode::InvalidFeeRate
        );

        self.fee_basis_points = new_fee_basis_points;
        self.updated_at = clock.unix_timestamp;
        Ok(())
    }

    pub fn calculate_fee(&self, amount: u64) -> Result<u64> {
        if self.fee_basis_points == 0 {
            return Ok(0);
        }

        require!(amount > 0, crate::ErrorCode::InvalidAmount);
        require!(amount <= self.max_transaction_amount, crate::ErrorCode::TransactionTooLarge);

        const MAX_SAFE_AMOUNT: u64 = u64::MAX / 10000;
        require!(amount <= MAX_SAFE_AMOUNT, crate::ErrorCode::FeeCalculationOverflow);

        let fee_calculation = amount
            .checked_mul(self.fee_basis_points)
            .ok_or(crate::ErrorCode::FeeCalculationOverflow)?;

        let fee_with_rounding = fee_calculation
            .checked_add(9999)
            .ok_or(crate::ErrorCode::FeeCalculationOverflow)?;

        let raw_fee = fee_with_rounding
            .checked_div(10000)
            .ok_or(crate::ErrorCode::FeeCalculationOverflow)?;

        let min_fee = 1_000_000u64;
        let max_fee = 100_000_000u64;

        let bounded_fee = raw_fee.clamp(min_fee, max_fee);

        require!(bounded_fee < amount, crate::ErrorCode::FeeExceedsAmount);

        let fee_ratio = bounded_fee
            .checked_mul(10000)
            .and_then(|v| v.checked_div(amount))
            .ok_or(crate::ErrorCode::FeeCalculationOverflow)?;

        require!(fee_ratio <= 500, crate::ErrorCode::InvalidFeeRate);

        Ok(bounded_fee)
    }

    pub fn validate_transaction_limits(&self, amount: u64) -> Result<()> {
        require!(
            amount >= self.min_transaction_amount,
            crate::ErrorCode::TransactionTooSmall
        );
        require!(
            amount <= self.max_transaction_amount,
            crate::ErrorCode::TransactionTooLarge
        );
        Ok(())
    }

    pub fn validate_mint_allowed(&self, _mint: &Pubkey) -> Result<()> {
        match self.mint_policy {
            MintPolicy::AllowAll => Ok(()),
            MintPolicy::BlockAll => Err(crate::ErrorCode::MintNotAllowed.into()),
            MintPolicy::Allowlist => {
                // For now, reject all mints when allowlist is enabled
                // In production, this would check against a separate allowlist account
                Err(crate::ErrorCode::MintNotAllowed.into())
            }
        }
    }

    pub fn update_mint_policy(&mut self, new_policy: MintPolicy, clock: &Clock) -> Result<()> {
        self.mint_policy = new_policy;
        self.updated_at = clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(tenant_id: Pubkey)]
pub struct InitializeTenantConfig<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [b"tenant_config", tenant_id.as_ref()],
        bump,
        space = 8 + TenantConfig::INIT_SPACE
    )]
    pub tenant_config: Account<'info, TenantConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTenantConfig<'info> {
    #[account(
        mut,
        seeds = [b"tenant_config", tenant_config.tenant_id.as_ref()],
        bump,
        has_one = admin
    )]
    pub tenant_config: Account<'info, TenantConfig>,

    pub admin: Signer<'info>,
}