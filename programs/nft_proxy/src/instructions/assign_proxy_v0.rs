use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

use crate::state::{ProxyAssignmentV0, ProxyConfigV0};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AssignProxyArgsV0 {
  pub expiration_time: i64,
}

#[derive(Accounts)]
pub struct AssignProxyV0<'info> {
  /// CHECK: Paying the rent for proxys
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    // Either match the current proxy, or if it is being initialized we will set it later
    // This works because asset can only be Pubkey default when this is a new account
    constraint = current_proxy_assignment.asset == asset.key() || current_proxy_assignment.asset == Pubkey::default(),
    constraint = asset.supply == 1,
    constraint = asset.decimals == 0
  )]
  pub asset: Box<Account<'info, Mint>>,
  #[account(
    constraint = current_proxy_assignment.voter == approver.key() || match token_account.as_ref() {
      Some(token_account) => token_account.owner == approver.key(),
      None => false
    }
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
  pub token_account: Option<Account<'info, TokenAccount>>,
  pub proxy_config: Box<Account<'info, ProxyConfigV0>>,
  #[account(
    // We init if needed here because in the case of the original
    // position owner delegating, this wont yet exist.
    init_if_needed,
    payer = payer,
    // Note that voter part of the PDA key for the first proxy is Pubkey::default. This ensures
    // that the holder can freely transfer the NFT without an disruption to the line. There is
    // exactly 1 primary proxy per NFT.
    seeds = [b"proxy_assignment", proxy_config.key().as_ref(), asset.key().as_ref(), voter.key().as_ref()],
    space = ProxyAssignmentV0::INIT_SPACE + 60,
    bump,
    // You can only proxy when it is not currently proxied to someone else.
    // Recall the proxy before re-delegating if necessary.
    constraint = current_proxy_assignment.next_voter == Pubkey::default() || current_proxy_assignment.next_voter == recipient.key()
  )]
  pub current_proxy_assignment: Box<Account<'info, ProxyAssignmentV0>>,
  /// CHECK: The wallet being delegated to
  #[account(
    // No creating loops! Cannot delegate back to the original position owner
    // This is just best effort, loops don't break the system. Someone could, theoretically,
    // delegate to someone and then transfer that NFT to them.
    constraint = match token_account.as_ref() {
      Some(token_account) => recipient.key() != token_account.owner,
      None => true
    }
  )]
  pub recipient: AccountInfo<'info>,
  #[account(
    init_if_needed,
    payer = payer,
    seeds = [b"proxy_assignment", proxy_config.key().as_ref(), asset.key().as_ref(), recipient.key().as_ref()],
    space = ProxyAssignmentV0::INIT_SPACE + 60,
    bump,
  )]
  pub next_proxy_assignment: Box<Account<'info, ProxyAssignmentV0>>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AssignProxyV0>, args: AssignProxyArgsV0) -> Result<()> {
  require_gt!(args.expiration_time, 0, ErrorCode::ExpirationTimeInvalid);
  let curr_ts = Clock::get().unwrap().unix_timestamp;
  let total_proxy_time = args
    .expiration_time
    .checked_sub(curr_ts)
    .ok_or_else(|| error!(ErrorCode::ExpirationPast))?;

  require_gt!(
    ctx.accounts.proxy_config.max_proxy_time,
    total_proxy_time,
    ErrorCode::ExpirationExceedsMax
  );
  require_gt!(total_proxy_time, 0, ErrorCode::ExpirationTimeInvalid);

  if let Some(current_season) = ctx.accounts.proxy_config.get_current_season(curr_ts) {
    require_gt!(
      current_season.end,
      args.expiration_time,
      ErrorCode::ExpirationExceedsSeasonMax
    );
  }

  ctx
    .accounts
    .current_proxy_assignment
    .set_inner(ProxyAssignmentV0 {
      index: ctx.accounts.current_proxy_assignment.index,
      asset: ctx.accounts.asset.key(),
      proxy_config: ctx.accounts.proxy_config.key(),
      voter: ctx.accounts.voter.key(),
      next_voter: ctx.accounts.recipient.key(),
      rent_refund: if ctx.accounts.current_proxy_assignment.rent_refund == Pubkey::default() {
        ctx.accounts.payer.key()
      } else {
        ctx.accounts.current_proxy_assignment.rent_refund
      },
      bump_seed: ctx.bumps["current_proxy_assignment"],
      expiration_time: if ctx.accounts.current_proxy_assignment.expiration_time > 0 {
        ctx.accounts.current_proxy_assignment.expiration_time
      } else {
        args.expiration_time
      },
    });

  require_gte!(
    ctx.accounts.current_proxy_assignment.expiration_time,
    args.expiration_time,
    ErrorCode::ExpirationExceedsPreceedingProxy
  );

  ctx
    .accounts
    .next_proxy_assignment
    .set_inner(ProxyAssignmentV0 {
      index: ctx.accounts.current_proxy_assignment.index + 1,
      asset: ctx.accounts.asset.key(),
      proxy_config: ctx.accounts.proxy_config.key(),
      voter: ctx.accounts.recipient.key(),
      next_voter: Pubkey::default(),
      rent_refund: if ctx.accounts.next_proxy_assignment.rent_refund == Pubkey::default() {
        ctx.accounts.payer.key()
      } else {
        ctx.accounts.next_proxy_assignment.rent_refund
      },
      bump_seed: ctx.bumps["next_proxy_assignment"],
      expiration_time: args.expiration_time,
    });

  Ok(())
}
