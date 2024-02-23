use anchor_lang::prelude::*;
use solana_program::{secp256k1_recover::{secp256k1_recover, Secp256k1Pubkey}, hash::hash};

use crate::error::JogoError;

#[account]
pub struct Admin {
    bump: u8,
    owner: Pubkey,
    recover_id: u8,
    signer: Secp256k1Pubkey,
}

impl Admin {
    pub fn verify_and_get_randomness(&self, digest: &[u8], sig: &[u8]) -> Result<[u8; 32]> {
        let pubkey = secp256k1_recover(digest, self.recover_id, sig)
            .map_err(|_| JogoError::Secp256k1RecoverError.into())?;
        if pubkey == self.signer {
            Ok(hash(sig).to_bytes())
        } else {
            Err(JogoError::InvalidSecp256k1Signature.into())
        }
    }
}
