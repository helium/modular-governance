use crate::state::*;
use anchor_lang::prelude::*;
use nft_delegation::state::DelegationConfigV0;

#[derive(Accounts)]
pub struct UpdateNftVoterV0<'info> {
  pub authority: Signer<'info>,
  pub delegation_config: Option<Account<'info, DelegationConfigV0>>,
  /// CHECK: Set if setting the new auth
  pub new_authority: Option<AccountInfo<'info>>,
  #[account(
    has_one = authority
  )]
  pub nft_voter: Box<Account<'info, NftVoterV0>>,
}

pub fn handler(ctx: Context<UpdateNftVoterV0>) -> Result<()> {
  if let Some(delegation_config) = &ctx.accounts.delegation_config {
    ctx.accounts.nft_voter.delegation_config = delegation_config.key();
  }

  if let Some(authority) = &ctx.accounts.new_authority {
    ctx.accounts.nft_voter.authority = authority.key()
  }

  Ok(())
}
