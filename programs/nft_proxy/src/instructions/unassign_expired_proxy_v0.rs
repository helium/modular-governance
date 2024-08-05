use crate::error::ErrorCode;
use anchor_lang::prelude::*;

use crate::state::*;

// Allow anyone to permissionlessly close expired proxies and refund to users
#[derive(Accounts)]
pub struct UnassignExpiredProxyV0<'info> {
  /// CHECK: Receiving rent for closing
  #[account(mut)]
  pub rent_refund: AccountInfo<'info>,
  pub asset: Box<Account<'info, Mint>>,
  #[account(
    mut,
    has_one = proxy_config,
    has_one = asset,
    constraint = prev_proxy_assignment.next_voter == proxy_assignment.voter,
    constraint = prev_proxy_assignment.asset == proxy_assignment.asset,
  )]
  pub prev_proxy_assignment: Box<Account<'info, ProxyAssignmentV0>>,
  #[account(
    mut,
    close = rent_refund,
    has_one = proxy_config,
    has_one = rent_refund,
    constraint = proxy_assignment.index > prev_proxy_assignment.index,
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
