use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

use crate::error::JogoError;

#[account]
pub struct Vault {
    config: VaultConfig,
    liquidity: u64,
    stake: u64,
    minted_lp: u64,
}

impl Vault {

    pub(crate) fn deposit(&mut self, amount: u64) -> u64 {
        let mint_lp = if self.liquidity > 0 {
            ((self.minted_lp as u128) * (amount as u128) / (self.liquidity as u128)) as u64
        } else {
            amount
        };
        self.liquidity += amount;
        self.minted_lp += mint_lp;

        mint_lp
    }

    pub(crate) fn withdraw(&mut self, amount: u64) -> Result<u64> {
        if (amount > self.liquidity) {
            Err(JogoError::InsufficientLiquidity.into())
        } else {
            let burn_lp =
                ((self.minted_lp as u128) * (amount as u128) / (self.liquidity as u128)) as u64;
            self.liquidity -= amount;
            self.minted_lp -= burn_lp;

            Ok(burn_lp)
        }
    }

    pub(crate) fn bet(&mut self, amount: u64) {
        self.stake += amount
    }

    pub(crate) fn settle(&mut self, mut amount: u64) -> Result<()> {
        if (amount > self.stake) {
            amount -= self.stake;
            if (amount > self.liquidity) {
                return Err(JogoError::InsufficientLiquidity.into())
            }
            self.stake = 0;
            self.liquidity -= amount;
        } else {
            self.stake -= amount;
        }

        Ok(())
    }
}

#[derive(Clone, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct VaultConfig {
    admin: Pubkey,
    mint: Pubkey,
    supply_account: Pubkey,
}