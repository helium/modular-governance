use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

use crate::state::{ProxyAssignmentV0, ProxyConfigV0};

// Anyone who helds a `ProxyV0` on this asset
// may undelegate it, so long as they have an earlier proxy index than or equal the position
// This means someone can also undelegate from themselves
#[derive(Accounts)]
pub struct UnassignProxyV0<'info> {
  /// CHECK: Receiving rent for closing
  #[account(mut)]
  pub rent_refund: AccountInfo<'info>,
  #[account(
    constraint = asset.supply == 1,
    constraint = asset.decimals == 0
  )]
  pub asset: Box<Account<'info, Mint>>,
  #[account(
    constraint = token_account.owner == approver.key() || current_proxy_assignment.voter == approver.key()
  )]
  pub approver: Signer<'info>,
  /// CHECK: This is the voter of the current proxy. May be the same as approver,
  /// or in the case of a primary proxy (first in the line), Pubkey::default
  #[account(
    constraint = (current_proxy_assignment.index != 0 && current_proxy_assignment.voter == voter.key())
             || (current_proxy_assignment.index == 0 && voter.key() == Pubkey::default())
  )]
  pub voter: AccountInfo<'info>,
  #[account(
    constraint = token_account.mint == asset.key(),
    constraint = token_account.amount == 1,
  )]
  pub token_account: Box<Account<'info, TokenAccount>>,
  #[account(
    has_one = proxy_config,
    has_one = voter,
    has_one = asset,
  )]
  pub current_proxy_assignment: Box<Account<'info, ProxyAssignmentV0>>,
  #[account(
    mut,
    has_one = proxy_config,
    constraint = prev_proxy_assignment.next_voter == proxy_assignment.voter,
    constraint = prev_proxy_assignment.asset == current_proxy_assignment.asset,
  )]
  pub prev_proxy_assignment: Box<Account<'info, ProxyAssignmentV0>>,
  #[account(
    mut,
    close = rent_refund,
    has_one = proxy_config,
    has_one = rent_refund,
    constraint = proxy_assignment.index >= current_proxy_assignment.index,
    constraint = proxy_assignment.asset == current_proxy_assignment.asset,
    constraint = proxy_assignment.next_voter == Pubkey::default(),
  )]
  pub proxy_assignment: Box<Account<'info, ProxyAssignmentV0>>,
  pub proxy_config: Box<Account<'info, ProxyConfigV0>>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UnassignProxyV0>) -> Result<()> {
  ctx.accounts.prev_proxy_assignment.next_voter = Pubkey::default();

  Ok(())
}
