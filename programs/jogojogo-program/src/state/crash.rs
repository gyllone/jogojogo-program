use anchor_lang::prelude::*;
use rust_decimal::Decimal;
use solana_program::clock::UnixTimestamp;

use crate::{error::JogoError, math::JogoDecimal};

#[account]
pub struct CrashConfig {
    admin: Pubkey,
    win_num: u64,
    win_denom: u64,
}

impl CrashConfig {
    pub fn new(admin: Pubkey, win_num: u64, win_denom: u64) -> Result<Self> {
        if (win_num == 0) || (win_denom == 0) || (win_num >= win_denom) {
            Err(JogoError::InvalidCrashConfig.into())
        } else {
            Ok(Self {
                admin,
                win_num,
                win_denom,
            })
        }
    }
}

#[account]
pub struct CrashGame {
    config: Pubkey,
    round: u64,
    bet_deadline: UnixTimestamp,
    random_seed: [u8; 32],
    randomness: Option<u32>,
}

impl CrashGame {
    pub(crate) fn new(
        config: Pubkey,
        round: u64,
        bet_deadline: UnixTimestamp,
        random_seed: [u8; 32],
    ) -> Self {
        Self {
            config,
            round,
            bet_deadline,
            random_seed,
            randomness: None,
        }
    }

    pub(crate) fn set_randomness(&mut self, randomness: &[u8]) -> Result<()> {
        if self.randomness.is_some() {
            Err(JogoError::RandomnessAlreadySet.into())
        } else {
            self.randomness = Some(
                u32::from_le_bytes(<[u8; 4]>::try_from(randomness).unwrap())
            );

            Ok(())
        }
    }

    pub(crate) fn compute_crash_point(&self, config: &CrashConfig) -> Result<u32> {
        let randomness = self.randomness.ok_or(JogoError::RandomnessNotSet)?;
        let num = (u32::MAX as u128) * (config.win_num as u128);
        let denom = (randomness as u128 + 1) * (config.win_denom as u128);

        Ok((num / denom) as u32)
    }
}

pub struct CrashBet {
    round: u64,
    stake: u64,
    number: u32,
}

impl CrashBet {
    pub fn new(round: u64, stake: u64, number: u32) -> Self {
        Self {
            round,
            stake,
            number,
        }
    }
}

