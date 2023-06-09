use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct OrganizationV0 {
  pub num_proposals: u32,
  /// Authority to create proposals under this organization
  pub authority: Pubkey,
  pub default_vote_controller: Pubkey,
  pub default_state_controller: Pubkey,
  pub default_on_vote_hook: Pubkey,
  pub proposal_program: Pubkey,
  #[max_len(200)]
  pub name: String,
  pub bump_seed: u8,
}

#[macro_export]
macro_rules! organization_seeds {
  ( $organization:expr ) => {
    &[
      b"organization",
      $organization.name.as_bytes(),
      &[$organization.bump_seed],
    ]
  };
}
