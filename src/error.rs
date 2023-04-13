use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Individual Ownership must be greater than 0.")]
    InvalidIndividualOwnership {},

    #[error("Total Ownership must equal 100%.")]
    InvalidTotalOwnership {},
}
