use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Proposal was already resolved. Call resolve_v0")]
  ProposalAlreadyResolved,
}
