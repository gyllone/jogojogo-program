use anchor_lang::prelude::*;

declare_id!("G4pgWxx2YTE2rGezW3cULwbAruCjh1TNYTu2Df6HCVP2");

#[program]
pub mod jogojogo_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
