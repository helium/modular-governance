use anchor_lang::prelude::*;

declare_id!("resL1j3p3QXAD2oQWW14Uv18iJrfoAwrCd3qTd2QDyj");

pub mod errors;
pub mod instructions;
pub mod state;

pub use instructions::*;

#[program]
pub mod state_controller {
  use super::*;

  pub fn on_vote_v0(ctx: Context<OnVoteV0>, args: proposal::VoteArgsV0) -> Result<()> {
    on_vote_v0::handler(ctx, args)
  }

  pub fn initialize_resolution_settings_v0(
    ctx: Context<InitializeResolutionSettingsV0>,
    args: InitializeResolutionSettingsArgsV0,
  ) -> Result<()> {
    initialize_resolution_settings_v0::handler(ctx, args)
  }

  pub fn start_vote_v0(ctx: Context<StartVoteV0>) -> Result<()> {
    start_vote_v0::handler(ctx)
  }
}

#[derive(Accounts)]
pub struct Initialize {}
