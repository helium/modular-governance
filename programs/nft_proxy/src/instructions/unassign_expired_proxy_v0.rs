use crate::error::ErrorCode;
use anchor_lang::prelude::*;

use crate::state::*;

// Allow anyone to permissionlessly close expired proxies and refund to users
#[derive(Accounts)]
pub struct UnassignExpiredProxyV0<'info> {
  /// CHECK: Receiving rent for closing
  #[account(mut)]
  pub rent_refund: AccountInfo<'info>,
  #[account(
    constraint = asset.supply == 1,
    constraint = asset.decimals == 0
  )]
  pub asset: Box<Account<'info, Mint>>,
  #[account(
    constraint = token_account.mint == asset.key(),
    constraint = token_account.amount == 1,
  )]
  pub token_account: Option<Account<'info, TokenAccount>>,
  #[account(
    has_one = proxy_config,
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
    constraint = proxy_assignment.expiration_time < Clock::get().unwrap().unix_timestamp @ ErrorCode::ExpirationNotPast,
  )]
  pub proxy_assignment: Box<Account<'info, ProxyAssignmentV0>>,
  pub proxy_config: Box<Account<'info, ProxyConfigV0>>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UnassignExpiredProxyV0>) -> Result<()> {
  ctx.accounts.prev_proxy_assignment.next_voter = Pubkey::default();

  Ok(())
}
