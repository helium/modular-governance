use crate::state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeResolutionSettingsArgsV0 {
  pub name: String,
  pub settings: ResolutionStrategy,
}

#[derive(Accounts)]
#[instruction(args: InitializeResolutionSettingsArgsV0)]
pub struct InitializeResolutionSettingsV0<'info> {
  /// CHECK: Payer
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    init,
    payer = payer,
    space = 8 + 60 + std::mem::size_of::<ResolutionSettingsV0>() + args.settings.nodes.iter().map(|node| node.size()).sum::<usize>(),
    seeds = [b"resolution_settings", args.name.as_bytes()],
    bump
  )]
  pub resolution_settings: Box<Account<'info, ResolutionSettingsV0>>,
  pub system_program: Program<'info, System>,
}

pub fn handler(
  ctx: Context<InitializeResolutionSettingsV0>,
  args: InitializeResolutionSettingsArgsV0,
) -> Result<()> {
  args.settings.validate()?;
  ctx
    .accounts
    .resolution_settings
    .set_inner(ResolutionSettingsV0 {
      bump_seed: ctx.bumps["resolution_settings"],
      settings: args.settings,
      name: args.name,
    });
  Ok(())
}
