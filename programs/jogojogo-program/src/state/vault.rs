use anchor_lang::prelude::*;

use crate::error::JogoError;

#[derive(Clone, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct VaultConfig {
    pub admin: Pubkey,
    pub lp_token_mint: Pubkey,
    pub supply_token_account: Pubkey,
}

#[account]
pub struct Vault {
    pub config: VaultConfig,
    pub liquidity: u64,
    pub stake: u64,
    pub minted_lp: u64,
}

impl Vault {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    pub fn supply(&self) -> u64 {
        return 0;
    }

    pub fn deposit(&mut self, amount: u64) -> Result<u64> {
        if (amount == 0) {
            return Err(JogoError::InvalidDepositAmount.into());
        }
        let mint_lp = if self.liquidity > 0 {
            ((self.minted_lp as u128) * (amount as u128) / (self.liquidity as u128)) as u64
        } else {
            amount
        };
        self.liquidity += amount;
        self.minted_lp += mint_lp;

        Ok(mint_lp)
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<u64> {
        if (amount == 0) {
            return Err(JogoError::InvalidWithdrawAmount.into());
        }
        let withdrawal = ((self.liquidity as u128) * (amount as u128) / (self.minted_lp as u128)) as u64
        if (withdrawal > self.liquidity) {
            return Err(JogoError::InsufficientLiquidity.into());
        }
        self.liquidity -= withdrawal;
        self.minted_lp -= amount;

        Ok(withdrawal)
    }

    pub fn bet(&mut self, amount: u64) {
        self.stake += amount
    }

    pub fn settle(&mut self, mut amount: u64) -> Result<()> {
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
