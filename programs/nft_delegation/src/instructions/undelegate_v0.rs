use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

use crate::state::DelegationV0;

// Anyone who helds a `DelegationV0` on this asset
// may undelegate it, so long as they have an earlier delegation index than or equal the position
// This means someone can also undelegate from themselves
#[derive(Accounts)]
pub struct UndelegateV0<'info> {
  /// CHECK: Receiving rent for closing
  #[account(mut)]
  pub rent_refund: AccountInfo<'info>,
  pub asset: Box<Account<'info, Mint>>,
  #[account(
    constraint = token_account.owner == approver.key() || current_delegation.owner == approver.key()
  )]
  pub approver: Signer<'info>,
  /// CHECK: This is the owner of the current delegation. May be the same as approver,
  /// or in the case of a primary delegation (first in the line), Pubkey::default
  #[account(
    constraint = (current_delegation.index != 0 && current_delegation.owner == owner.key())
             || (current_delegation.index == 0 && owner.key() == Pubkey::default())
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
