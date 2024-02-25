mod error;
mod math;
mod state;
mod instructions;

use anchor_lang::prelude::*;

pub use state::*;
pub use instructions::*;

declare_id!("G4pgWxx2YTE2rGezW3cULwbAruCjh1TNYTu2Df6HCVP2");

// #[program]
// pub mod jogojogo_program {
//     use super::*;
//
//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         Ok(())
//     }
// }
