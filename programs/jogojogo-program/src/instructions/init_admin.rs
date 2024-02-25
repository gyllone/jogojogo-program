use anchor_lang::prelude::*;
use solana_program::secp256k1_recover::{SECP256K1_PUBLIC_KEY_LENGTH, Secp256k1Pubkey};

use crate::state::Admin;

#[derive(Accounts)]
#[instruction(recover_id: u8, signer: [u8; SECP256K1_PUBLIC_KEY_LENGTH])]
pub struct InitAdmin<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer = owner, space = Admin::SIZE)]
    pub admin: Account<'info, Admin>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn init_admin(
    ctx: Context<InitAdmin>,
    recover_id: u8,
    signer: [u8; SECP256K1_PUBLIC_KEY_LENGTH],
) -> Result<()> {
    ctx.accounts.admin.set_inner(
        Admin {
            owner: ctx.accounts.owner.key(),
            recover_id,
            signer: Secp256k1Pubkey(signer),
        }
    );

    Ok(())
}
