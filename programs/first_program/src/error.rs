use anchor_lang::prelude::*;

#[error_code]
pub enum CounterError {
    #[msg("Account is already initialized")]
    AlreadyInitialized,
    #[msg("Account is not initialized")]
    NotInitialized,
    #[msg("User cannot perform this action")]
    Forbidden,
    #[msg("Arithmetic error")]
    Arithmetic,
    #[msg("Invalid admin account provided")]
    InvalidAdmin,
}
