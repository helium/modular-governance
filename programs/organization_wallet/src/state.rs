use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct OrganizationWalletV0 {
  pub index: u16,
  pub organization: Pubkey,
  pub wallet: Pubkey,
  pub proposal_configs: Vec<Pubkey>,
  pub name: String,
  pub bump_seed: u8,
  pub wallet_bump_seed: u8,
}

#[macro_export]
macro_rules! organization_wallet_seeds {
  ( $wallet:expr ) => {
    &[
      b"organization_wallet",
      $wallet.organization.as_ref(),
      $wallet.index.as_bytes(),
      &[$wallet.bump_seed],
    ]
  };
}

#[macro_export]
macro_rules! wallet_seeds {
  ( $wallet:expr ) => {
    &[
      b"wallet",
      $wallet.organization.as_ref(),
      &$wallet.index.to_le_bytes(),
      &[$wallet.wallet_bump_seed],
    ]
  };
}

#[account]
#[derive(Default)]
pub struct WalletProposalV0 {
  pub proposal: Pubkey,
  pub organization_wallet: Pubkey,
  pub num_transactions_by_choice: Vec<u16>,
}

#[account]
#[derive(Default)]
pub struct ChoiceTransactionV0 {
  pub wallet_proposal: Pubkey,
  pub proposal: Pubkey,
  pub organization_wallet: Pubkey,
  pub choice_index: u16,
  // Cannot be executed until this offset from proposal end
  pub allow_execution_offset: u32,
  // Cannot be executed after this offset
  pub disable_execution_offset: u32,
  pub bump_seed: u8,
  pub transaction: CompiledTransactionV0,
}

#[macro_export]
macro_rules! wallet_proposal_seeds {
  ( $wallet_proposal:expr ) => {
    &[
      b"wallet_proposal",
      $wallet_proposal.organization_wallet.as_ref(),
      $wallet_proposal.proposal.as_ref(),
      &[$wallet_proposal.bump_seed],
    ]
  };
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CompiledInstructionV0 {
  /// Index into the transaction keys array indicating the program account that executes this instruction.
  pub program_id_index: u8,
  /// Ordered indices into the transaction keys array indicating which accounts to pass to the program.
  pub accounts: Vec<u8>,
  /// The program input data.
  pub data: Vec<u8>,
}

impl CompiledInstructionV0 {
  pub fn size(&self) -> usize {
    1 + self.accounts.len() + self.data.len()
  }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CompiledTransactionV0 {
  // Accounts are ordered as follows:
  // 1. Writable signer accounts
  // 2. Read only signer accounts
  // 3. writable accounts
  // 4. read only accounts
  pub num_rw_signers: u8,
  pub num_ro_signers: u8,
  pub num_rw: u8,
  pub accounts: Vec<Pubkey>,
  pub instructions: Vec<CompiledInstructionV0>,
  /// Additional signer seeds. Should include bump. Useful for things like initializing a mint where
  /// you cannot pass a keypair.
  /// Note that these seeds will be prefixed with "custom", org_wallet.index
  /// and the bump you pass and account should be consistent with this. But to save space
  /// in the instruction, they should be ommitted here. See tests for examples
  pub signer_seeds: Vec<Vec<Vec<u8>>>,
}

#[account]
pub struct OrganizationWalletPropoalV0 {
  pub organization_wallet: Pubkey,
  pub proposal: Pubkey,
  pub accounts: Vec<Pubkey>,
  pub transactions_by_choice: Vec<Vec<CompiledTransactionV0>>,
  pub bump_seed: u8,
}

#[macro_export]
macro_rules! organization_wallet_proposal_seeds {
  ( $organization_wallet_proposal:expr ) => {
    &[
      b"organization_wallet_proposal",
      $organization_wallet_proposal.proposal.as_ref(),
      &[$organization_wallet_proposal.bump_seed],
    ]
  };
}
