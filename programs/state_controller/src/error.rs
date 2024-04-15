use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Proposal was already resolved. Call resolve_v0")]
  ProposalAlreadyResolved,
  #[msg("Resolved choices must not be empty")]
  ChoicesEmpty,
  #[msg("End timestamp has already passed")]
  EndTimestampPassed,
  #[msg("Offset must be a positive integer")]
  InvalidOffset,
  #[msg("Percentage may not be less than 0 or greater than PERCENTAGE_DIVISOR")]
  InvalidPercentage,
  #[msg("Top n must be greater than 0")]
  InvalidTopN,
}
