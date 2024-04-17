use crate::state::{ProxyConfigV0, SeasonV0};
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeProxyConfigArgsV0 {
  pub name: String,
  pub max_proxy_time: i64,
  pub seasons: Vec<SeasonV0>,
}

#[derive(Accounts)]
#[instruction(args: InitializeProxyConfigArgsV0)]
pub struct InitializeProxyConfigV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  /// CHECK: The authority of this config
  pub authority: AccountInfo<'info>,
  #[account(
    init,
    payer = payer,
    space = 60 + std::mem::size_of::<ProxyConfigV0>() + args.name.len() + args.seasons.len() * 8,
    seeds = [b"proxy_config".as_ref(), args.name.as_bytes()],
    bump
  )]
  pub proxy_config: Account<'info, ProxyConfigV0>,
  pub system_program: Program<'info, System>,
}

pub fn handler(
  ctx: Context<InitializeProxyConfigV0>,
  args: InitializeProxyConfigArgsV0,
) -> Result<()> {
  ctx.accounts.proxy_config.set_inner(ProxyConfigV0 {
    authority: ctx.accounts.authority.key(),
    name: args.name,
    max_proxy_time: args.max_proxy_time,
    seasons: args.seasons,
  });

  Ok(())
}
