
// Importamos las funcionalidades necesarias de Anchor
use anchor_lang::prelude::*;

// Definimos el módulo principal del programa
#[program]
pub mod contrato_meme {
    use super::*;

    // Función para inicializar el token
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let meme_data = &mut ctx.accounts.meme_data;
        
        // Establecemos el nombre del token
        meme_data.name = "Codeinu".to_string();
        
        // Establecemos el símbolo del token
        meme_data.symbol = "$CINU".to_string();
        
        // Establecemos el número de decimales (9 es común en Solana)
        meme_data.decimals = 9;
        
        // Establecemos el suministro total (100 mil millones con 9 decimales)
        meme_data.total_supply = 100_000_000_000 * 10u64.pow(9);
        
        // Establecemos la autoridad del token
        meme_data.authority = ctx.accounts.authority.key();
        
        Ok(())
    }

    // Función para acuñar nuevos tokens
    pub fn mint(ctx: Context<Mint>, amount: u64) -> Result<()> {
        let meme_data = &mut ctx.accounts.meme_data;
        let recipient = &mut ctx.accounts.recipient;

        // Verificamos que solo la autoridad pueda acuñar tokens
        if *ctx.accounts.authority.key != meme_data.authority {
            return Err(ErrorCode::Unauthorized.into());
        }

        // Verificamos que no se exceda el suministro total
        if meme_data.total_supply + amount > 100_000_000_000 * 10u64.pow(meme_data.decimals as u32) {
            return Err(ErrorCode::ExceedsSupply.into());
        }

        // Actualizamos el saldo del destinatario
        recipient.balance = recipient.balance.checked_add(amount).unwrap();

        // Actualizamos el suministro total
        meme_data.total_supply = meme_data.total_supply.checked_add(amount).unwrap();

        Ok(())
    }

    // Función para transferir tokens entre cuentas
    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        let from = &mut ctx.accounts.from;
        let to = &mut ctx.accounts.to;

        // Verificamos que el remitente tenga suficientes fondos
        if from.balance < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        // Realizamos la transferencia
        from.balance = from.balance.checked_sub(amount).unwrap();
        to.balance = to.balance.checked_add(amount).unwrap();

        // Emitimos un evento de transferencia
        emit!(TransferEvent {
            from: *from.to_account_info().key,
            to: *to.to_account_info().key,
            amount,
        });

        Ok(())
    }

    // Nueva función para quemar tokens
    pub fn burn(ctx: Context<Burn>, amount: u64) -> Result<()> {
        let meme_data = &mut ctx.accounts.meme_data;
        let from = &mut ctx.accounts.from;

        // Verificamos que la cuenta tenga suficientes fondos
        if from.balance < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        // Quemamos los tokens
        from.balance = from.balance.checked_sub(amount).unwrap();
        meme_data.total_supply = meme_data.total_supply.checked_sub(amount).unwrap();

        // Emitimos un evento de quemado
        emit!(BurnEvent {
            from: *from.to_account_info().key,
            amount,
        });

        Ok(())
    }

    // Nueva función para cambiar la autoridad del token
    pub fn change_authority(ctx: Context<ChangeAuthority>, new_authority: Pubkey) -> Result<()> {
        let meme_data = &mut ctx.accounts.meme_data;

        // Verificamos que solo la autoridad actual pueda cambiar la autoridad
        if *ctx.accounts.authority.key != meme_data.authority {
            return Err(ErrorCode::Unauthorized.into());
        }

        // Cambiamos la autoridad
        meme_data.authority = new_authority;

        // Emitimos un evento de cambio de autoridad
        emit!(ChangeAuthorityEvent {
            old_authority: *ctx.accounts.authority.key,
            new_authority,
        });

        Ok(())
    }

    // Función para hacer stake de tokens
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;
        let stake_account = &mut ctx.accounts.stake_account;
        let clock = Clock::get()?;

        // Verificar que el usuario tenga suficientes tokens
        if token_account.balance < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        // Transferir tokens de la cuenta de token a la cuenta de stake
        token_account.balance = token_account.balance.checked_sub(amount).unwrap();
        stake_account.amount = stake_account.amount.checked_add(amount).unwrap();
        stake_account.start_time = clock.unix_timestamp;

        // Emitir evento de stake
        emit!(StakeEvent {
            user: *ctx.accounts.user.key,
            amount,
        });

        Ok(())
    }

    // Función para retirar tokens del stake
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;
        let stake_account = &mut ctx.accounts.stake_account;
        let clock = Clock::get()?;

        // Calcular la recompensa (por ejemplo, 1% por día)
        let days_staked = (clock.unix_timestamp - stake_account.start_time) / 86400; // 86400 segundos en un día
        let reward = (stake_account.amount as u128 * days_staked as u128 * 1) / 100;

        // Transferir tokens y recompensa de vuelta a la cuenta de token
        token_account.balance = token_account.balance
            .checked_add(stake_account.amount)
            .unwrap()
            .checked_add(reward as u64)
            .unwrap();

        // Emitir evento de unstake
        emit!(UnstakeEvent {
            user: *ctx.accounts.user.key,
            amount: stake_account.amount,
            reward: reward as u64,
        });

        // Reiniciar la cuenta de stake
        stake_account.amount = 0;
        stake_account.start_time = 0;

        Ok(())
    }
}

// Estructura para la inicialización del token
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32 + 1 + 8 + 32)]
    pub meme_data: Account<'info, MemeData>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Estructura para la acuñación de tokens
#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(mut)]
    pub meme_data: Account<'info, MemeData>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
}

// Estructura para la transferencia de tokens
#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
}

// Nueva estructura para quemar tokens
#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(mut)]
    pub meme_data: Account<'info, MemeData>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
}

// Nueva estructura para cambiar la autoridad
#[derive(Accounts)]
pub struct ChangeAuthority<'info> {
    #[account(mut)]
    pub meme_data: Account<'info, MemeData>,
    pub authority: Signer<'info>,
}

// Estructura para el stake de tokens
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

// Estructura para el unstake de tokens
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

// Estructura para almacenar los datos del token
#[account]
pub struct MemeData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u64,
    pub authority: Pubkey,
}

// Estructura para representar una cuenta de token
#[account]
pub struct TokenAccount {
    pub owner: Pubkey,
    pub balance: u64,
}

// Estructura para almacenar información de staking
#[account]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub amount: u64,
    pub start_time: i64,
}

// Enumeración de códigos de error
#[error_code]
pub enum ErrorCode {
    #[msg("No autorizado para realizar esta acción")]
    Unauthorized,
    #[msg("La cantidad excede el suministro máximo")]
    ExceedsSupply,
    #[msg("Fondos insuficientes para la operación")]
    InsufficientFunds,
}

// Evento que se emite cuando se realiza una transferencia
#[event]
pub struct TransferEvent {
    #[index]
    pub from: Pubkey,
    #[index]
    pub to: Pubkey,
    pub amount: u64,
}

// Nuevo evento que se emite cuando se queman tokens
#[event]
pub struct BurnEvent {
    #[index]
    pub from: Pubkey,
    pub amount: u64,
}

// Nuevo evento que se emite cuando se cambia la autoridad
#[event]
pub struct ChangeAuthorityEvent {
    #[index]
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
}

// Evento que se emite cuando se hace stake de tokens
#[event]
pub struct StakeEvent {
    #[index]
    pub user: Pubkey,
    pub amount: u64,
}

// Evento que se emite cuando se hace unstake de tokens
#[event]
pub struct UnstakeEvent {
    #[index]
    pub user: Pubkey,
    pub amount: u64,
    pub reward: u64,
}