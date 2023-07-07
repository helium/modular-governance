use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Already voted for this choice")]
  AlreadyVoted,
  #[msg("Exceeded max choices")]
  MaxChoicesExceeded,
  #[msg("No vote to relinquish for this choice")]
  NoVoteForThisChoice,
}
