use anchor_lang::prelude::*;

use crate::error::JogoError;

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub struct Fraction {
    numerator: u64,
    denominator: u64,
}

impl Fraction {
    pub fn new(numerator: u64, denominator: u64) -> Result<Self> {
        if (denominator == 0) {
            Err(JogoError::InvalidFraction.into())
        } else {
            Ok(Self {
                numerator,
                denominator,
            })
        }
    }

    pub fn zero() -> Self {
        Self {
            numerator: 0,
            denominator: 1,
        }
    }

    pub fn one() -> Self {
        Self {
            numerator: 1,
            denominator: 1,
        }
    }

    pub fn try_mul(self, other: Self) -> Result<Self> {
        let numerator = self.numerator.checked_mul(other.numerator);
        let denominator = self.denominator.checked_mul(other.denominator);
        match (numerator, denominator) {
            (Some(numerator), Some(denominator)) => {
                Self::new(numerator, denominator)
            }
            _ => Err(JogoError::FractionOverflow.into())
        }
    }

    pub fn mul_u64(self, other: u64) -> u64 {
        (self.numerator as u128 * other / self.denominator as u128) as u64
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        let left = self.numerator as u128 * other.denominator;
        let right = self.denominator as u128 * other.numerator;
        left == right
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left = self.numerator as u128 * other.denominator;
        let right = self.denominator as u128 * other.numerator;
        left.partial_cmp(&right)
    }
}
