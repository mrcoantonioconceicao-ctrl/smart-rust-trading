use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod smart_vault {
    use super::*;

    /// Inicializa o cofre on-chain do Agente Nolan com parâmetros corporativos e seguros
    pub fn initialize_vault(ctx: Context<InitializeVault>, bump: u8) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.bump = bump;
        vault.balance_usdc = 50_000_000; // Representação de 50.00 USDC (6 casas decimais)
        vault.is_active = true;
        msg!("[SMART-VAULT] Cofre on-chain inicializado com sucesso. Saldo: 50.00 USDC");
        Ok(())
    }

    /// Executa uma ordem de trade validada on-chain através do cofre PDA
    pub fn execute_trade(ctx: Context<ExecuteTrade>, amount: u64, is_buy: bool) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(vault.is_active, VaultError::VaultInactive);

        if is_buy {
            require!(vault.balance_usdc >= amount, VaultError::InsufficientFunds);
            vault.balance_usdc -= amount;
            msg!("[SMART-VAULT] Trade de COMPRA executado on-chain. Montante: {} unidades", amount);
        } else {
            vault.balance_usdc += amount;
            msg!("[SMART-VAULT] Trade de VENDA executado on-chain. Montante: {} unidades", amount);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [b"smart_vault", authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteTrade<'info> {
    #[account(
        mut,
        seeds = [b"smart_vault", authority.key().as_ref()],
        bump = vault.bump,
        has_one = authority
    )]
    pub vault: Account<'info, VaultState>,
    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub authority: Pubkey,
    pub bump: u8,
    pub balance_usdc: u64,
    pub is_active: bool,
}

#[error_code]
pub enum VaultError {
    #[msg("O cofre encontra-se inativo ou bloqueado por política de segurança.")]
    VaultInactive,
    #[msg("Fundos insuficientes no cofre on-chain para executar a operação.")]
    InsufficientFunds,
}
