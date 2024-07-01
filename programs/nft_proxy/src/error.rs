use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("The specified expiration time exceeds the maximum allowed for this proxy configuration")]
  ExpirationExceedsMax,
  #[msg("The specified expiration time exceeds the maximum allowed for this season")]
  ExpirationExceedsSeasonMax,
  #[msg("The specified expiration time has already passed")]
  ExpirationPast,
  #[msg("The specified expiration time exceeds the expiration of the existing delegatio")]
  ExpirationExceedsPreceedingProxy,
  #[msg("The seasons are not sorted")]
  SeasonsNotSorted,
  #[msg("The data size increase is not valid")]
  InvalidDataIncrease,
  #[msg("The expiration time is invalid")]
  ExpirationTimeInvalid,
}
