use anchor_lang::prelude::*;

use crate::{Modify, error::CounterError};

pub fn decrement_handler(ctx: Context<Modify>) -> Result<()> {
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

    if let Some(result) = ctx.accounts.counter.value.checked_sub(1) {
        ctx.accounts.counter.value = result;
    } else {
        return Err(CounterError::Arithmetic.into());
    }

    Ok(())
}
