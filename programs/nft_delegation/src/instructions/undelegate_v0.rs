use anchor_lang::prelude::*;

use crate::state::DelegationV0;

// Anyone who helds a `DelegationV0` on this asset
// may undelegate it, so long as they have an earlier delegation index than or equal the position
// This means someone can also undelegate from themselves
#[derive(Accounts)]
pub struct UndelegateV0<'info> {
  /// CHECK: Receiving rent for closing
  #[account(mut)]
  pub rent_refund: AccountInfo<'info>,
  pub owner: Signer<'info>,
  #[account(
    has_one = owner,
  )]
  pub current_delegation: Box<Account<'info, DelegationV0>>,
  #[account(
    mut,
    constraint = prev_delegation.next_owner == delegation.owner,
    constraint = prev_delegation.asset == current_delegation.asset,
  )]
  pub prev_delegation: Box<Account<'info, DelegationV0>>,
  #[account(
    mut,
    close = rent_refund,
    has_one = rent_refund,
    constraint = delegation.index >= current_delegation.index,
    constraint = delegation.asset == current_delegation.asset
  )]
  pub delegation: Box<Account<'info, DelegationV0>>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UndelegateV0>) -> Result<()> {
  ctx.accounts.prev_delegation.next_owner = Pubkey::default();

  Ok(())
}
