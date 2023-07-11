use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct TokenVoterV0 {
  pub authority: Pubkey,
  pub deposit_mint: Pubkey,
  /// NFT collection that all receipts will be a part of
  pub collection: Pubkey,
  #[max_len(32)]
  pub name: String,
  pub bump_seed: u8,
}

#[account]
#[derive(Default, InitSpace)]
pub struct ReceiptV0 {
  pub token_voter: Pubkey,
  pub mint: Pubkey,
  pub amount: u64,
  pub num_active_votes: u64,
  pub bump_seed: u8,
}

#[account]
#[derive(Default)]
pub struct VoteMarkerV0 {
  pub voter: Pubkey,
  pub token_voter: Pubkey,
  pub proposal: Pubkey,
  pub mint: Pubkey,
  pub choices: Vec<u16>,
  pub bump_seed: u8,
}

#[macro_export]
macro_rules! token_voter_seeds {
  ($token_voter:expr) => {
    &[
      b"token_voter".as_ref(),
      $token_voter.name.as_bytes(),
      &[$token_voter.bump_seed],
    ]
  };
}

#[macro_export]
macro_rules! receipt_seeds {
  ($receipt:expr) => {
    &[
      b"receipt".as_ref(),
      $receipt.mint.as_ref(),
      &[$receipt.bump_seed],
    ]
  };
}
