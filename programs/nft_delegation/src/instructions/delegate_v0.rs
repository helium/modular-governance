use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

use crate::state::DelegationV0;

#[derive(Accounts)]
pub struct DelegateV0<'info> {
  /// CHECK: Paying the rent for delegations
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    // Either match the current delegation, or if it is being initialized we will set it later
    // This works because asset can only be Pubkey default when this is a new account
    constraint = current_delegation.asset == mint.key() || current_delegation.asset == Pubkey::default()
  )]
  pub mint: Box<Account<'info, Mint>>,
  #[account(
    constraint = token_account.owner == owner.key() || current_delegation.owner == owner.key()
  )]
  pub owner: Signer<'info>,
  #[account(
    constraint = token_account.mint == mint.key(),
    constraint = token_account.amount == 1,
  )]
  pub token_account: Box<Account<'info, TokenAccount>>,
  #[account(
    // We init if needed here because in the case of the original
    // owner delegating, this wont yet exist.
    init_if_needed,
    payer = payer,
    seeds = [b"delegation", mint.key().as_ref(), owner.key().as_ref()],
    space = DelegationV0::INIT_SPACE + 60,
    bump,
    // You can only delegate when it is not currently delegated to someone else.
    // Recall the delegation before re-delegating if necessary.
    constraint = current_delegation.next_owner == Pubkey::default()
  )]
  pub current_delegation: Box<Account<'info, DelegationV0>>,
  /// CHECK: The wallet being delegated to
  pub recipient: AccountInfo<'info>,
  #[account(
    init,
    payer = payer,
    seeds = [b"delegation", mint.key().as_ref(), recipient.key().as_ref()],
    space = DelegationV0::INIT_SPACE + 60,
    bump,
  )]
  pub next_delegation: Box<Account<'info, DelegationV0>>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DelegateV0>) -> Result<()> {
  ctx.accounts.current_delegation.set_inner(DelegationV0 {
    index: ctx.accounts.current_delegation.index,
    asset: ctx.accounts.mint.key(),
    owner: ctx.accounts.owner.key(),
    next_owner: ctx.accounts.recipient.key(),
    rent_refund: ctx.accounts.payer.key(),
    bump_seed: ctx.bumps["current_delegation"],
  });
  ctx.accounts.next_delegation.set_inner(DelegationV0 {
    index: ctx.accounts.current_delegation.index + 1,
    asset: ctx.accounts.mint.key(),
    owner: ctx.accounts.recipient.key(),
    next_owner: Pubkey::default(),
    rent_refund: ctx.accounts.payer.key(),
    bump_seed: ctx.bumps["next_delegation"],
  });
  Ok(())
}
