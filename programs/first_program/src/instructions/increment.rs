use anchor_lang::{prelude::*, system_program};

use crate::{error::CounterError, CounterState};

const ADMIN_PUBKEY: Pubkey = Pubkey::from_str_const("admaFv4scM2FDN7gVJA8C1HMJtQv2YHkMPnShgXb7PY");
const INCREMENT_FEE: u64 = 1_000_000;

#[derive(Accounts)]
pub struct Modify<'info> {
    pub signer: Signer<'info>,

    /// CHECK: This is verified to be the exact admin address with anchor constraints.
    #[account(
        mut,
        address = ADMIN_PUBKEY @ CounterError::InvalidAdmin
    )]
    pub admin_account: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"counter", signer.key().as_ref()],
        bump
    )]
    pub counter: Account<'info, CounterState>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct IncrementLog {
    pub incrementer: Pubkey,
    pub previous_value: u64,
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

    msg!("hello world");
    emit!(IncrementLog {
        incrementer: ctx.accounts.signer.key(),
        previous_value: ctx.accounts.counter.value
    });

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.admin_account.to_account_info(),
        },
    );
    system_program::transfer(cpi_context, INCREMENT_FEE)?;

    if let Some(result) = ctx.accounts.counter.value.checked_add(1) {
        ctx.accounts.counter.value = result;
    } else {
        return Err(CounterError::Arithmetic.into());
    }

    Ok(())
}
