use anchor_lang::prelude::*;

use crate::{error::CounterError, CounterState};

#[derive(Accounts)]
pub struct Modify<'info> {
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"counter", signer.key().as_ref()],
        bump
    )]
    pub counter: Account<'info, CounterState>,
}

pub fn increment_handler(ctx: Context<Modify>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.counter.authority,
        ctx.accounts.signer.key(),
        CounterError::Forbidden
    );
    require_eq!(
        ctx.accounts.counter.version,
        1,
        CounterError::NotInitialized
    );

    if let Some(result) = ctx.accounts.counter.value.checked_add(1) {
        ctx.accounts.counter.value = result;
    } else {
        return Err(CounterError::Arithmetic.into())
    }

    Ok(())
}
