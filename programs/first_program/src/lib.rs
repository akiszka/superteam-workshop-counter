use anchor_lang::prelude::*;

mod error;
mod instructions;
mod state;

pub use instructions::*;
pub use state::*;

declare_id!("3ezVurLDjU5gy1DoavqNHKRskLJmxYVA1ZRsH2E1myti");

#[program]
pub mod first_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize_handler(ctx)
    }

    pub fn increment(ctx: Context<Modify>) -> Result<()> {
        increment_handler(ctx)
    }

    pub fn decrement(ctx: Context<Modify>) -> Result<()> {
        decrement_handler(ctx)
    }
}
