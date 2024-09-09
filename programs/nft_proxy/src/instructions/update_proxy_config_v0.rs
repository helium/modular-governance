use crate::error::ErrorCode;
use crate::resize_to_fit::resize_to_fit;
use crate::state::{ProxyConfigV0, SeasonV0};
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateProxyConfigArgsV0 {
  pub max_proxy_time: Option<i64>,
  pub seasons: Option<Vec<SeasonV0>>,
}

#[derive(Accounts)]
#[instruction(args: UpdateProxyConfigArgsV0)]
pub struct UpdateProxyConfigV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  /// CHECK: The authority of this config
  pub authority: AccountInfo<'info>,
  #[account(
    mut,
    has_one = authority
  )]
  pub proxy_config: Account<'info, ProxyConfigV0>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateProxyConfigV0>, args: UpdateProxyConfigArgsV0) -> Result<()> {
  if let Some(seasons) = args.seasons {
    if !seasons
      .iter()
      .zip(seasons.iter().skip(1))
      .all(|(a, b)| a.start <= b.start && a.end <= b.end)
    {
      return Err(error!(ErrorCode::SeasonsNotSorted));
    }

    ctx.accounts.proxy_config.seasons = seasons;
  }

  if let Some(max_proxy_time) = args.max_proxy_time {
    ctx.accounts.proxy_config.max_proxy_time = max_proxy_time;
  }

  resize_to_fit(
    &ctx.accounts.payer.to_account_info(),
    &ctx.accounts.system_program.to_account_info(),
    &ctx.accounts.proxy_config,
  )?;

  Ok(())
}
