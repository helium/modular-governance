use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct DelegationConfigV0 {
  pub authority: Pubkey,
  #[max_len(32)]
  pub name: String,
  // Max time to delegate in seconds
  pub max_delegation_time: i64,
}
// Forms an on chain linked list of delegations
#[account]
#[derive(Default, InitSpace)]
pub struct DelegationV0 {
  pub owner: Pubkey,
  pub delegation_config: Pubkey,
  pub asset: Pubkey,
  // Current index in the delegation chain.
  pub index: u16,
  // If this is the last in the line, Pubkey::default.
  pub next_owner: Pubkey,
  // The address of the account that paid for rent. Rent on closing
  // delegations always goes to the key who originally paid for them.
  pub rent_refund: Pubkey,
  // Unix timestamp in seconds that this delegation expires
  pub expiration_time: i64,
  pub bump_seed: u8,
}
