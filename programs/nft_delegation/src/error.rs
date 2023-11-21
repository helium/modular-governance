use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg(
    "The specified expiration time exceeds the maximum allowed for this delegation configuration"
  )]
  ExpirationExceedsMax,
  #[msg("The specified expiration time exceeds the maximum allowed for this season")]
  ExpirationExceedsSeasonMax,
  #[msg("The specified expiration time has already passed")]
  ExpirationPast,
  #[msg("The specified expiration time exceeds the expiration of the existing delegatio")]
  ExpirationExceedsPreceedingDelegation,
}
