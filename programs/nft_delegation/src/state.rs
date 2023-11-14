use anchor_lang::prelude::*;

// Forms an on chain linked list of delegations
#[account]
#[derive(Default, InitSpace)]
pub struct DelegationV0 {
  pub owner: Pubkey,
  pub asset: Pubkey,
  // Current index in the delegation chain.
  pub index: u16,
  // If this is the last in the line, Pubkey::default.
  pub next_owner: Pubkey,
  // The address of the account that paid for rent. Rent on closing
  // delegations always goes to the key who originally paid for them.
  pub rent_refund: Pubkey,
  pub bump_seed: u8,
}
