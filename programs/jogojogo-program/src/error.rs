use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum JogoError {
    #[msg("Invalid fraction")]
    InvalidFraction,
    #[msg("Fraction overflow")]
    FractionOverflow,
    #[msg("Failed to recover secp256k1 signature")]
    Secp256k1RecoverError,
    #[msg("Invalid secp256k1 signature")]
    InvalidSecp256k1Signature,
    #[msg("Randomness already set")]
    RandomnessAlreadySet,
    #[msg("Randomness not set")]
    RandomnessNotSet,
    #[msg("Invalid winning rate")]
    InvalidWinningRate,
    #[msg("Invalid odd")]
    InvalidOdd,
    #[msg("Invalid deposit amount")]
    InvalidDepositAmount,
    #[msg("Invalid withdraw amount")]
    InvalidWithdrawAmount,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
}