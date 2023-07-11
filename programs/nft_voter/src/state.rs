use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct NftVoterV0 {
  pub authority: Pubkey,
  pub collection: Pubkey,
  #[max_len(32)]
  pub name: String,
  pub bump_seed: u8,
}

#[account]
#[derive(Default)]
pub struct VoteMarkerV0 {
  pub voter: Pubkey,
  pub nft_voter: Pubkey,
  pub proposal: Pubkey,
  pub mint: Pubkey,
  pub choices: Vec<u16>,
  pub bump_seed: u8,
}

#[macro_export]
macro_rules! nft_voter_seeds {
  ($token_voter:expr) => {
    &[
      b"nft_voter".as_ref(),
      $token_voter.name.as_bytes(),
      &[$token_voter.bump_seed],
    ]
  };
}
