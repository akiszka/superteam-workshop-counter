use anchor_lang::prelude::*;

use crate::{error::CounterError, CounterState};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + CounterState::INIT_SPACE,
        seeds = [b"counter", signer.key().as_ref()],
        bump
    )]
    pub counter: Account<'info, CounterState>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_handler(ctx: Context<Initialize>) -> Result<()> {
    require_eq!(
        ctx.accounts.counter.version,
        0,
        CounterError::AlreadyInitialized
    );

    ctx.accounts.counter.authority = ctx.accounts.signer.key();
    ctx.accounts.counter.version = 1;

    Ok(())
}
