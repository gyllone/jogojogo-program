use anchor_lang::prelude::*;
use solana_program::clock::UnixTimestamp;

use crate::{error::JogoError, math::Fraction};

#[account]
pub struct CrashConfig {
    pub admin: Pubkey,
    pub win_rate: Fraction,
    pub max_odd: Fraction,
}

impl CrashConfig {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    pub fn new(admin: Pubkey, win_rate: Fraction, max_odd: Fraction) -> Result<Self> {
        if (win_rate >= Fraction::one() || win_rate == Fraction::zero()) {
            return Err(JogoError::InvalidWinningRate.into());
        }
        if (max_odd <= Fraction::one()) {
            return Err(JogoError::InvalidOdd.into());
        }

        Ok(Self {
            admin,
            win_rate,
            max_odd,
        })
    }

    pub fn set_win_rate(&mut self, win_rate: Fraction) -> Result<()> {
        if (win_rate >= Fraction::one() || win_rate == Fraction::zero()) {
            return Err(JogoError::InvalidWinningRate.into());
        }
        self.win_rate = win_rate;

        Ok(())
    }

    pub fn set_max_odd(&mut self, max_odd: Fraction) -> Result<()> {
        if (max_odd <= Fraction::one()) {
            return Err(JogoError::InvalidOdd.into());
        }
        self.max_odd = max_odd;

        Ok(())
    }
}

#[account]
pub struct CrashGame {
    pub config: Pubkey,
    pub round: u64,
    pub bet_deadline: UnixTimestamp,
    pub random_seed: [u8; 32],
    pub randomness: Option<u32>,
}

impl CrashGame {
    pub const SIZE: usize = 1 + std::mem::size_of::<Self>();

    #[inline]
    pub fn new(
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

    pub fn get_deadline(&self) -> UnixTimestamp {
        return self.bet_deadline
    }

    pub fn set_randomness(&mut self, randomness: &[u8]) -> Result<()> {
        if self.randomness.is_some() {
            Err(JogoError::RandomnessAlreadySet.into())
        } else {
            self.randomness = Some(
                u32::from_le_bytes(<[u8; 4]>::try_from(randomness).unwrap())
            );

            Ok(())
        }
    }

    pub fn compute_crash_point(&self, config: &CrashConfig) -> Result<Fraction> {
        let randomness = self.randomness.ok_or(JogoError::RandomnessNotSet)? as u64;
        let scale = Fraction::new(1u64 << 32, randomness + 1)?;
        config.win_rate.try_mul(scale).map(
            |p| if p > config.max_odd { config.max_odd } else { p }
        )
    }
}

#[account]
pub struct CrashBet {
    pub owner: Pubkey,
    pub round: u64,
    pub stake: u64,
    pub reserve: u64,
}

impl CrashBet {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    #[inline]
    pub fn new(owner: Pubkey, round: u64, stake: u64, reserve: u64) -> Self {
        Self { owner, round, stake, reserve }
    }

    #[inline]
    pub fn get_reserve(&self) -> u64 {
        self.reserve
    }
}

#[account]
pub struct CrashSettle {
    pub owner: Pubkey,
    pub round: u64,
    pub point: Option<Fraction>,
}

impl CrashSettle {
    pub const SIZE: usize = 1 + std::mem::size_of::<Self>();

    #[inline]
    pub fn new(owner: Pubkey, round: u64, point: Option<Fraction>) -> Self {
        Self { owner, round, point }
    }

    pub fn settle(&self, crash_point: Fraction, bet: &CrashBet) -> u64 {
        if let Some(point) = self.point {
            if point <= crash_point {
                return point.mul_u64(bet.stake);
            }
        }
        0
    }
}
