use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount, Transfer, MintTo, transfer, mint_to},
    associated_token::AssociatedToken,
};

use crate::state::Vault;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(seeds = [b"authority", vault.config.admin.as_ref()], bump)]
    pub admin_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut, address = vault.config.lp_token_mint)]
    pub lp_token_mint: Account<'info, Mint>,
    #[account(mut, address = vault.config.supply_token_account)]
    pub supply_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_deposit_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = lp_token_mint,
        associated_token::authority = user,
    )]
    pub user_lp_token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

pub(crate) fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_deposit_token_account.to_account_info(),
            to: ctx.accounts.supply_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    transfer(cpi_ctx, amount)?;

    let minted_lp = ctx.accounts.vault.deposit(amount)?;
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.lp_token_mint.to_account_info(),
            to: ctx.accounts.user_lp_token_account.to_account_info(),
            authority: ctx.accounts.admin_authority.to_account_info(),
        },
        &[
            &[
                b"authority",
                ctx.accounts.vault.config.admin.as_ref(),
                &[ctx.bumps.admin_authority],
            ],
        ],
    );
    mint_to(cpi_ctx, minted_lp)
}