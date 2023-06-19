use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct OrganizationWalletV0 {
  pub index: u16,
  pub organization: Pubkey,
  pub wallet: Pubkey,
  pub proposal_config: Pubkey,
  #[max_len(200)]
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
  pub bump_seed: u8,
  pub choice_transactions: Vec<Vec<CompiledTransaction>>,
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
pub struct CompiledInstruction {
  /// Index into the transaction keys array indicating the program account that executes this instruction.
  pub program_id_index: u8,
  /// Ordered indices into the transaction keys array indicating which accounts to pass to the program.
  pub accounts: Vec<u8>,
  /// The program input data.
  pub data: Vec<u8>,
}

impl CompiledInstruction {
  pub fn size(&self) -> usize {
    1 + self.accounts.len() + self.data.len()
  }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CompiledTransaction {
  pub accounts: Vec<Pubkey>,
  pub instructions: Vec<CompiledInstruction>,
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
  pub transactions_by_choice: Vec<Vec<CompiledTransaction>>,
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
