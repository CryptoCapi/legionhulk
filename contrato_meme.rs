use anchor_lang::prelude::*;

declare_id!("LegionHulk111111111111111111111111111111111"); // Cambia esto al programId real cuando despliegues

const MAX_SUPPLY: u64 = 10_000_000_000 * 10u64.pow(9); // 10 billones con 9 decimales
const INITIAL_SUPPLY: u64 = 5_000_000_000 * 10u64.pow(9); // 5 billones iniciales

#[program]
pub mod legion_hulk {
    use super::*;

    // Inicializar el token con supply inicial
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let meme_data = &mut ctx.accounts.meme_data;

        meme_data.name = "Legion Hulk".to_string();
        meme_data.symbol = "LGHK".to_string();
        meme_data.decimals = 9;
        meme_data.total_supply = INITIAL_SUPPLY;
        meme_data.authority = ctx.accounts.authority.key();

        // Crear cuenta de token del creador con el supply inicial
        let creator_account = &mut ctx.accounts.creator_account;
        creator_account.owner = ctx.accounts.authority.key();
        creator_account.balance = INITIAL_SUPPLY;

        Ok(())
    }

    // Acuñar nuevos tokens (hasta el máximo)
    pub fn mint(ctx: Context<Mint>, amount: u64) -> Result<()> {
        let meme_data = &mut ctx.accounts.meme_data;
        let recipient = &mut ctx.accounts.recipient;

        if *ctx.accounts.authority.key != meme_data.authority {
            return Err(ErrorCode::Unauthorized.into());
        }

        if meme_data.total_supply + amount > MAX_SUPPLY {
            return Err(ErrorCode::ExceedsSupply.into());
        }

        recipient.balance = recipient.balance.checked_add(amount).unwrap();
        meme_data.total_supply = meme_data.total_supply.checked_add(amount).unwrap();

        Ok(())
    }

    // Transferencia simple
    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        let from = &mut ctx.accounts.from;
        let to = &mut ctx.accounts.to;

        if from.balance < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        from.balance = from.balance.checked_sub(amount).unwrap();
        to.balance = to.balance.checked_add(amount).unwrap();

        emit!(TransferEvent {
            from: *from.to_account_info().key,
            to: *to.to_account_info().key,
            amount,
        });

        Ok(())
    }

    // Quemar tokens
    pub fn burn(ctx: Context<Burn>, amount: u64) -> Result<()> {
        let meme_data = &mut ctx.accounts.meme_data;
        let from = &mut ctx.accounts.from;

        if from.balance < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        from.balance = from.balance.checked_sub(amount).unwrap();
        meme_data.total_supply = meme_data.total_supply.checked_sub(amount).unwrap();

        emit!(BurnEvent {
            from: *from.to_account_info().key,
            amount,
        });

        Ok(())
    }

    // Cambiar autoridad
    pub fn change_authority(ctx: Context<ChangeAuthority>, new_authority: Pubkey) -> Result<()> {
        let meme_data = &mut ctx.accounts.meme_data;

        if *ctx.accounts.authority.key != meme_data.authority {
            return Err(ErrorCode::Unauthorized.into());
        }

        meme_data.authority = new_authority;

        emit!(ChangeAuthorityEvent {
            old_authority: *ctx.accounts.authority.key,
            new_authority,
        });

        Ok(())
    }

    // Staking
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;
        let stake_account = &mut ctx.accounts.stake_account;
        let clock = Clock::get()?;

        if token_account.balance < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        token_account.balance = token_account.balance.checked_sub(amount).unwrap();
        stake_account.amount = stake_account.amount.checked_add(amount).unwrap();
        stake_account.start_time = clock.unix_timestamp;

        emit!(StakeEvent {
            user: *ctx.accounts.user.key,
            amount,
        });

        Ok(())
    }

    // Unstake + recompensa 1% diario
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;
        let stake_account = &mut ctx.accounts.stake_account;
        let clock = Clock::get()?;

        let days_staked = (clock.unix_timestamp - stake_account.start_time) / 86400;
        let reward = (stake_account.amount as u128 * days_staked as u128 * 1) / 100;

        token_account.balance = token_account
            .balance
            .checked_add(stake_account.amount)
            .unwrap()
            .checked_add(reward as u64)
            .unwrap();

        emit!(UnstakeEvent {
            user: *ctx.accounts.user.key,
            amount: stake_account.amount,
            reward: reward as u64,
        });

        stake_account.amount = 0;
        stake_account.start_time = 0;

        Ok(())
    }
}

// --- Accounts ---
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32 + 1 + 8 + 32)]
    pub meme_data: Account<'info, MemeData>,
    #[account(init, payer = authority, space = 8 + 32 + 8)]
    pub creator_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(mut)]
    pub meme_data: Account<'info, MemeData>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(mut)]
    pub meme_data: Account<'info, MemeData>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ChangeAuthority<'info> {
    #[account(mut)]
    pub meme_data: Account<'info, MemeData>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8 + 8,
        seeds = [b"stake", user.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"stake", user.key().as_ref()],
        bump,
        close = user
    )]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// --- Data structures ---
#[account]
pub struct MemeData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u64,
    pub authority: Pubkey,
}

#[account]
pub struct TokenAccount {
    pub owner: Pubkey,
    pub balance: u64,
}

#[account]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub amount: u64,
    pub start_time: i64,
}

// --- Errors ---
#[error_code]
pub enum ErrorCode {
    #[msg("No autorizado para realizar esta acción")]
    Unauthorized,
    #[msg("La cantidad excede el suministro máximo")]
    ExceedsSupply,
    #[msg("Fondos insuficientes para la operación")]
    InsufficientFunds,
}

// --- Eventos ---
#[event]
pub struct TransferEvent {
    #[index]
    pub from: Pubkey,
    #[index]
    pub to: Pubkey,
    pub amount: u64,
}

#[event]
pub struct BurnEvent {
    #[index]
    pub from: Pubkey,
    pub amount: u64,
}

#[event]
pub struct ChangeAuthorityEvent {
    #[index]
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
}

#[event]
pub struct StakeEvent {
    #[index]
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct UnstakeEvent {
    #[index]
    pub user: Pubkey,
    pub amount: u64,
    pub reward: u64,
}
