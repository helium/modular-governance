use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

use crate::state::{ProxyConfigV0, ProxyV0};

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
    constraint = current_proxy.asset == asset.key() || current_proxy.asset == Pubkey::default(),
    constraint = asset.supply == 1,
    constraint = asset.decimals == 0
  )]
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
  pub proxy_config: Box<Account<'info, ProxyConfigV0>>,
  #[account(
    // We init if needed here because in the case of the original
    // owner delegating, this wont yet exist.
    init_if_needed,
    payer = payer,
    // Note that owner part of the PDA key for the first proxy is Pubkey::default. This ensures
    // that the holder can freely transfer the NFT without an disruption to the line. There is
    // exactly 1 primary proxy per NFT.
    seeds = [b"proxy", proxy_config.key().as_ref(), asset.key().as_ref(), owner.key().as_ref()],
    space = ProxyV0::INIT_SPACE + 60,
    bump,
    // You can only delegate when it is not currently delegated to someone else.
    // Recall the proxy before re-delegating if necessary.
    constraint = current_proxy.next_owner == Pubkey::default()
  )]
  pub current_proxy: Box<Account<'info, ProxyV0>>,
  /// CHECK: The wallet being delegated to
  #[account(
    // No creating loops! Cannot delegate back to the original owner
    // This is just best effort, loops don't break the system. Someone could, theoretically,
    // delegate to someone and then transfer that NFT to them.
    constraint = recipient.key() != token_account.owner
  )]
  pub recipient: AccountInfo<'info>,
  #[account(
    init,
    payer = payer,
    seeds = [b"proxy", proxy_config.key().as_ref(), asset.key().as_ref(), recipient.key().as_ref()],
    space = ProxyV0::INIT_SPACE + 60,
    bump,
  )]
  pub next_proxy: Box<Account<'info, ProxyV0>>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AssignProxyV0>, args: AssignProxyArgsV0) -> Result<()> {
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

  if let Some(current_season_end) = ctx.accounts.proxy_config.get_current_season_end(curr_ts) {
    require_gt!(
      current_season_end,
      args.expiration_time,
      ErrorCode::ExpirationExceedsSeasonMax
    );
  }

  ctx.accounts.current_proxy.set_inner(ProxyV0 {
    index: ctx.accounts.current_proxy.index,
    asset: ctx.accounts.asset.key(),
    proxy_config: ctx.accounts.proxy_config.key(),
    owner: ctx.accounts.owner.key(),
    next_owner: ctx.accounts.recipient.key(),
    rent_refund: ctx.accounts.payer.key(),
    bump_seed: ctx.bumps["current_proxy"],
    expiration_time: if ctx.accounts.current_proxy.expiration_time > 0 {
      ctx.accounts.current_proxy.expiration_time
    } else {
      args.expiration_time
    },
  });

  require_gte!(
    ctx.accounts.current_proxy.expiration_time,
    args.expiration_time,
    ErrorCode::ExpirationExceedsPreceedingProxy
  );

  ctx.accounts.next_proxy.set_inner(ProxyV0 {
    index: ctx.accounts.current_proxy.index + 1,
    asset: ctx.accounts.asset.key(),
    proxy_config: ctx.accounts.proxy_config.key(),
    owner: ctx.accounts.recipient.key(),
    next_owner: Pubkey::default(),
    rent_refund: ctx.accounts.payer.key(),
    bump_seed: ctx.bumps["next_proxy"],
    expiration_time: args.expiration_time,
  });

  Ok(())
}
