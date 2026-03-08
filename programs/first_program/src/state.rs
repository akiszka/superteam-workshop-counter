use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct CounterState {
    pub authority: Pubkey,
    pub value: u64,
    pub version: u8,
}