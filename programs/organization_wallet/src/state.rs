use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct OrganizationWalletV0 {
  pub organization: Pubkey,
  pub wallet: Pubkey,
  pub vote_controller: Pubkey,
  pub state_controller: Pubkey,
  pub on_vote_hook: Pubkey,
  #[max_len(200)]
  pub name: String,
  pub bump_seed: u8,
  pub wallet_bump_seed: u8,
}

#[macro_export]
macro_rules! organization_wallet_seeds {
  ( $wallet:expr ) => {
    &[
      b"organization-wallet",
      $wallet.name.as_bytes(),
      &[$wallet.bump_seed],
    ]
  };
}

#[macro_export]
macro_rules! wallet_seeds {
  ( $wallet:expr ) => {
    &[
      b"wallet",
      $wallet.name.as_bytes(),
      &[$wallet.wallet_bump_seed],
    ]
  };
}

pub struct WalletProposalV0 {
  pub proposal: Pubkey,
  pub organization_wallet: Pubkey,
  pub accounts: Vec<Pubkey>,
  pub instructions: Vec<CompiledInstruction>,
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
