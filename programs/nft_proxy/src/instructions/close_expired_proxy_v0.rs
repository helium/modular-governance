use crate::error::ErrorCode;
use anchor_lang::prelude::*;

use crate::state::*;

// Allow anyone to permissionlessly close expired proxies at 0 index and refund to users
#[derive(Accounts)]
pub struct CloseExpiredProxyV0<'info> {
  /// CHECK: Receiving rent for closing
  #[account(mut)]
  pub rent_refund: AccountInfo<'info>,
  #[account(
    mut,
    close = rent_refund,
    has_one = rent_refund,
    constraint = proxy_assignment.index == 0,
    constraint = proxy_assignment.next_voter == Pubkey::default(),
    constraint = proxy_assignment.expiration_time < Clock::get().unwrap().unix_timestamp @ ErrorCode::ExpirationNotPast,
  )]
  pub proxy_assignment: Box<Account<'info, ProxyAssignmentV0>>,
  pub system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<CloseExpiredProxyV0>) -> Result<()> {
  Ok(())
}
