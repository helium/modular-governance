use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

use crate::state::ProxyV0;

// Anyone who helds a `ProxyV0` on this asset
// may undelegate it, so long as they have an earlier proxy index than or equal the position
// This means someone can also undelegate from themselves
#[derive(Accounts)]
pub struct UnassignProxyV0<'info> {
  /// CHECK: Receiving rent for closing
  #[account(mut)]
  pub rent_refund: AccountInfo<'info>,
  pub asset: Box<Account<'info, Mint>>,
  #[account(
    constraint = token_account.owner == approver.key() || current_proxy.owner == approver.key()
  )]
  pub approver: Signer<'info>,
  /// CHECK: This is the owner of the current proxy. May be the same as approver,
  /// or in the case of a primary proxy (first in the line), Pubkey::default
  #[account(
    constraint = (current_proxy.index != 0 && current_proxy.owner == owner.key())
             || (current_proxy.index == 0 && owner.key() == Pubkey::default())
  )]
  pub owner: AccountInfo<'info>,
  #[account(
    constraint = token_account.mint == asset.key(),
    constraint = token_account.amount == 1,
  )]
  pub token_account: Box<Account<'info, TokenAccount>>,
  #[account(
    has_one = owner,
    has_one = asset,
  )]
  pub current_proxy: Box<Account<'info, ProxyV0>>,
  #[account(
    mut,
    constraint = prev_proxy.next_owner == proxy.owner,
    constraint = prev_proxy.asset == current_proxy.asset,
  )]
  pub prev_proxy: Box<Account<'info, ProxyV0>>,
  #[account(
    mut,
    close = rent_refund,
    has_one = rent_refund,
    constraint = proxy.index >= current_proxy.index,
    constraint = proxy.asset == current_proxy.asset
  )]
  pub proxy: Box<Account<'info, ProxyV0>>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UnassignProxyV0>) -> Result<()> {
  ctx.accounts.prev_proxy.next_owner = Pubkey::default();

  Ok(())
}
