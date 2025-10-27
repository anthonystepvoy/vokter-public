use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint as MintInterface, TokenAccount as TokenAccountInterface, TokenInterface};
use anchor_spl::associated_token::AssociatedToken;

// SPL Token Vault Extension for Vokter Guardian Protocol
// Extends existing Guardian 2FA protection to SPL tokens like $TRUMP
// *** SPL_VAULT.RS UPDATED AT 14:15 - TOKEN INTERFACES + TENANT CONFIG ***

#[derive(Accounts)]
#[instruction(tenant_id: Pubkey, mint: Pubkey)]
pub struct InitializeSPLVault<'info> {
    /// Main Vokter wallet - must exist and have Guardian setup
    #[account(
        seeds = [
            b"vokter_wallet".as_ref(), 
            owner.key().as_ref()
        ],
        bump,
        has_one = owner,
        has_one = guardian
    )]
    pub vokter_wallet: Account<'info, crate::VokterWallet>,

    /// Tenant configuration
    #[account(
        seeds = [b"tenant_config", tenant_id.as_ref()],
        bump,
        constraint = tenant_config.is_active @ crate::ErrorCode::TenantNotActive
    )]
    pub tenant_config: Account<'info, crate::TenantConfig>,

    #[account(mut)]
    pub owner: Signer<'info>,
    
    /// Guardian signature required for SPL vault creation
    pub guardian: Signer<'info>,

    /// The SPL token mint to create vault for (Token or Token-2022)
    pub mint: InterfaceAccount<'info, MintInterface>,

    /// SPL vault account (PDA: wallet + mint)
    #[account(
        init,
        payer = owner,
        seeds = [
            b"spl_vault".as_ref(),
            vokter_wallet.key().as_ref(),
            mint.key().as_ref()
        ],
        bump,
        space = 8 + SPLVault::INIT_SPACE
    )]
    pub spl_vault: Account<'info, SPLVault>,

    /// Associated token account for the vault (owned by vault PDA)
    #[account(
        init,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = spl_vault,
        associated_token::token_program = token_program
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccountInterface>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(tenant_id: Pubkey)]
pub struct DepositSPLTokens<'info> {
    /// Main Vokter wallet
    #[account(
        seeds = [
            b"vokter_wallet".as_ref(), 
            owner.key().as_ref()
        ],
        bump,
        has_one = owner
    )]
    pub vokter_wallet: Account<'info, crate::VokterWallet>,

    /// Tenant configuration
    #[account(
        seeds = [b"tenant_config", tenant_id.as_ref()],
        bump,
        constraint = tenant_config.is_active @ crate::ErrorCode::TenantNotActive
    )]
    pub tenant_config: Account<'info, crate::TenantConfig>,

    #[account(mut)]
    pub owner: Signer<'info>,

    /// SPL vault account
    #[account(
        mut,
        seeds = [
            b"spl_vault".as_ref(),
            vokter_wallet.key().as_ref(),
            mint.key().as_ref()
        ],
        bump,
        has_one = mint,
        has_one = vault_token_account
    )]
    pub spl_vault: Account<'info, SPLVault>,

    pub mint: InterfaceAccount<'info, MintInterface>,

    /// Owner's token account (source)
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program
    )]
    pub owner_token_account: InterfaceAccount<'info, TokenAccountInterface>,

    /// Vault's token account (destination)
    #[account(mut)]
    pub vault_token_account: InterfaceAccount<'info, TokenAccountInterface>,

    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
#[instruction(tenant_id: Pubkey)]
pub struct WithdrawSPLTokens<'info> {
    /// Main Vokter wallet - GUARDIAN SIGNATURE REQUIRED
    #[account(
        seeds = [
            b"vokter_wallet".as_ref(), 
            owner.key().as_ref()
        ],
        bump,
        has_one = owner,
        has_one = guardian  // CRITICAL: Guardian signature enforces 2FA
    )]
    pub vokter_wallet: Account<'info, crate::VokterWallet>,

    /// Tenant configuration
    #[account(
        seeds = [b"tenant_config", tenant_id.as_ref()],
        bump,
        constraint = tenant_config.is_active @ crate::ErrorCode::TenantNotActive
    )]
    pub tenant_config: Account<'info, crate::TenantConfig>,

    #[account(mut)]
    pub owner: Signer<'info>,
    
    /// Guardian signature required - this enforces 2FA protection
    pub guardian: Signer<'info>,

    /// SPL vault account
    #[account(
        mut,
        seeds = [
            b"spl_vault".as_ref(),
            vokter_wallet.key().as_ref(),
            mint.key().as_ref()
        ],
        bump,
        has_one = mint,
        has_one = vault_token_account
    )]
    pub spl_vault: Account<'info, SPLVault>,

    pub mint: InterfaceAccount<'info, MintInterface>,

    /// Vault's token account (source)
    #[account(mut)]
    pub vault_token_account: InterfaceAccount<'info, TokenAccountInterface>,

    /// Recipient's token account (destination)
    #[account(
        mut,
        token::mint = mint,
        token::token_program = token_program
    )]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccountInterface>,

    pub token_program: Interface<'info, TokenInterface>,
}

/// SPL Token Vault State Account
#[account]
#[derive(InitSpace)]
pub struct SPLVault {
    /// Reference to main Vokter wallet
    pub vokter_wallet: Pubkey,
    
    /// SPL token mint address (e.g., $TRUMP mint)
    pub mint: Pubkey,
    
    /// Associated token account for this vault
    pub vault_token_account: Pubkey,
    
    /// Vault creation timestamp
    pub created_at: i64,
    
    /// Total deposits (for analytics)
    pub total_deposited: u64,
    
    /// Total withdrawals (for analytics)
    pub total_withdrawn: u64,
    
    /// Last activity timestamp
    pub last_activity: i64,
    
    /// Vault status
    pub status: SPLVaultStatus,
    
    /// Emergency pause flag
    pub is_paused: bool,
    
    /// Reserved for future features
    pub _reserved: [u8; 32],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum SPLVaultStatus {
    Active,
    Paused,
    Closed,
}

impl anchor_lang::Space for SPLVaultStatus {
    const INIT_SPACE: usize = 1; // Enum discriminant size
}

impl Default for SPLVaultStatus {
    fn default() -> Self {
        SPLVaultStatus::Active
    }
}

/// Initialize SPL Token Vault
/// Creates Guardian-protected vault for specific SPL token
pub fn initialize_spl_vault(
    ctx: Context<InitializeSPLVault>, 
    _tenant_id: Pubkey,
    _mint: Pubkey
) -> Result<()> {
    // Validate mint is allowed by tenant policy
    ctx.accounts.tenant_config.validate_mint_allowed(&ctx.accounts.mint.key())?;
    
    let clock = Clock::get()?;
    
    let spl_vault = &mut ctx.accounts.spl_vault;
    spl_vault.vokter_wallet = ctx.accounts.vokter_wallet.key();
    spl_vault.mint = ctx.accounts.mint.key();
    spl_vault.vault_token_account = ctx.accounts.vault_token_account.key();
    spl_vault.created_at = clock.unix_timestamp;
    spl_vault.total_deposited = 0;
    spl_vault.total_withdrawn = 0;
    spl_vault.last_activity = clock.unix_timestamp;
    spl_vault.status = SPLVaultStatus::Active;
    spl_vault.is_paused = false;

    emit!(SPLVaultInitialized {
        wallet: ctx.accounts.vokter_wallet.key(),
        spl_vault: spl_vault.key(),
        mint: ctx.accounts.mint.key(),
        vault_token_account: ctx.accounts.vault_token_account.key(),
        owner: ctx.accounts.owner.key(),
        guardian: ctx.accounts.guardian.key(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

/// Deposit SPL Tokens to Vault (No 2FA required - securing tokens)
pub fn deposit_spl_tokens(
    ctx: Context<DepositSPLTokens>, 
    _tenant_id: Pubkey,
    amount: u64
) -> Result<()> {
    require!(amount > 0, crate::VokterWalletError::InvalidAmount);
    
    // Validate mint is allowed by tenant policy
    ctx.accounts.tenant_config.validate_mint_allowed(&ctx.accounts.mint.key())?;
    
    let clock = Clock::get()?;
    let spl_vault = &mut ctx.accounts.spl_vault;
    
    // Check vault is active
    require!(
        spl_vault.status == SPLVaultStatus::Active && !spl_vault.is_paused,
        crate::VokterWalletError::VaultPaused
    );

    // Transfer tokens to Guardian-protected vault using token interface
    anchor_spl::token_interface::transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token_interface::TransferChecked {
                from: ctx.accounts.owner_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.vault_token_account.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;

    // Update vault state
    spl_vault.total_deposited = spl_vault.total_deposited.checked_add(amount).unwrap();
    spl_vault.last_activity = clock.unix_timestamp;

    emit!(SPLTokensDeposited {
        wallet: spl_vault.vokter_wallet,
        spl_vault: spl_vault.key(),
        mint: spl_vault.mint,
        owner: ctx.accounts.owner.key(),
        amount,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

/// Withdraw SPL Tokens from Vault (REQUIRES GUARDIAN 2FA)
/// This is where the Guardian magic happens - needs both signatures!
pub fn withdraw_spl_tokens(
    ctx: Context<WithdrawSPLTokens>, 
    _tenant_id: Pubkey,
    amount: u64
) -> Result<()> {
    require!(amount > 0, crate::VokterWalletError::InvalidAmount);
    
    // Validate mint is allowed by tenant policy
    ctx.accounts.tenant_config.validate_mint_allowed(&ctx.accounts.mint.key())?;
    
    let clock = Clock::get()?;
    let spl_vault = &mut ctx.accounts.spl_vault;
    
    // Check vault is active
    require!(
        spl_vault.status == SPLVaultStatus::Active && !spl_vault.is_paused,
        crate::VokterWalletError::VaultPaused
    );

    // Check sufficient balance
    let vault_balance = ctx.accounts.vault_token_account.amount;
    require!(vault_balance >= amount, crate::VokterWalletError::InsufficientFunds);

    // 🔥 CRITICAL SECURITY: Guardian signature enforced by Anchor constraints
    // This means Guardian service validated TOTP and provided co-signature!

    // Create signer seeds for vault PDA to authorize transfer
    let wallet_key = spl_vault.vokter_wallet;
    let mint_key = spl_vault.mint;
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"spl_vault",
        wallet_key.as_ref(),
        mint_key.as_ref(),
        &[ctx.bumps.spl_vault],
    ]];

    // Transfer tokens from Guardian vault to recipient using token interface
    anchor_spl::token_interface::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token_interface::TransferChecked {
                from: ctx.accounts.vault_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: spl_vault.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;

    // Update vault state
    spl_vault.total_withdrawn = spl_vault.total_withdrawn.checked_add(amount).unwrap();
    spl_vault.last_activity = clock.unix_timestamp;

    emit!(SPLTokensWithdrawn {
        wallet: spl_vault.vokter_wallet,
        spl_vault: spl_vault.key(),
        mint: spl_vault.mint,
        owner: ctx.accounts.owner.key(),
        guardian: ctx.accounts.guardian.key(),
        recipient: ctx.accounts.recipient_token_account.owner,
        amount,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// Events for SPL vault operations
#[event]
pub struct SPLVaultInitialized {
    pub wallet: Pubkey,
    pub spl_vault: Pubkey,
    pub mint: Pubkey,
    pub vault_token_account: Pubkey,
    pub owner: Pubkey,
    pub guardian: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct SPLTokensDeposited {
    pub wallet: Pubkey,
    pub spl_vault: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct SPLTokensWithdrawn {
    pub wallet: Pubkey,
    pub spl_vault: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub guardian: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}