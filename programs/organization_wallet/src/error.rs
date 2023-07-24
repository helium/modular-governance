use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("The realloc increase was too large")]
  InvalidDataIncrease,
  InstructionSerializeFailed,
  #[msg("Account passed that should be a signer")]
  InvalidSigner,
  #[msg("Writable setting on account is invalid")]
  InvalidWritable,
  #[msg("Invalid account key for index")]
  InvalidAccount,
  #[msg("Transactiion has already been executed")]
  TransactionAlreadyExecuted,
  InvalidTransactionIndex,
  InvalidProposalState,
  InvalidOrganization,
  InvalidProposalConfig,
}
