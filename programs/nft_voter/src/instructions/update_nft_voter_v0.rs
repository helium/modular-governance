use crate::state::*;
use anchor_lang::prelude::*;
use nft_proxy::state::ProxyConfigV0;

#[derive(Accounts)]
pub struct UpdateNftVoterV0<'info> {
  pub authority: Signer<'info>,
  pub proxy_config: Option<Account<'info, ProxyConfigV0>>,
  /// CHECK: Set if setting the new auth
  pub new_authority: Option<AccountInfo<'info>>,
  #[account(
    has_one = authority
  )]
  pub nft_voter: Box<Account<'info, NftVoterV0>>,
}

pub fn handler(ctx: Context<UpdateNftVoterV0>) -> Result<()> {
  if let Some(proxy_config) = &ctx.accounts.proxy_config {
    ctx.accounts.nft_voter.proxy_config = proxy_config.key();
  }

  if let Some(authority) = &ctx.accounts.new_authority {
    ctx.accounts.nft_voter.authority = authority.key()
  }

  Ok(())
}
