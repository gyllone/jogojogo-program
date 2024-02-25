use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};

use crate::state::{Admin, Vault, VaultConfig};

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(has_one = owner)]
    pub admin: Account<'info, Admin>,
    #[account(seeds = [b"authority", admin.key().as_ref()], bump)]
    pub admin_authority: UncheckedAccount<'info>,
    #[account(init, payer = owner, space = Vault::SIZE)]
    pub vault: Account<'info, Vault>,
    pub supply_token_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = owner,
        token::mint = supply_token_mint,
        token::authority = admin_authority,
    )]
    pub supply_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = owner,
        mint::decimals = supply_token_mint.decimals,
        mint::authority = admin_authority,
    )]
    pub lp_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn init_vault(ctx: Context<InitVault>) -> Result<()> {
    ctx.accounts.vault.set_inner(
        Vault {
            config: VaultConfig {
                admin: ctx.accounts.admin.key(),
                lp_token_mint: ctx.accounts.lp_token_mint.key(),
                supply_token_account: ctx.accounts.supply_token_account.key(),
            },
            liquidity: 0,
            stake: 0,
            minted_lp: 0,
        }
    );

    Ok(())
}