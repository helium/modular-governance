use anchor_lang::prelude::*;

declare_id!("stcfiqW3fwD9QCd8Bqr1NBLrs7dftZHBQe7RiMMA4aM");

pub mod error;
pub mod instructions;
pub mod state;

pub use instructions::*;

#[program]
pub mod state_controller {
  use super::*;

  pub fn on_vote_v0(ctx: Context<OnVoteV0>, args: VoteArgsV0) -> Result<Option<Vec<u16>>> {
    on_vote_v0::handler(ctx, args)
  }

  pub fn initialize_resolution_settings_v0(
    ctx: Context<InitializeResolutionSettingsV0>,
    args: InitializeResolutionSettingsArgsV0,
  ) -> Result<()> {
    initialize_resolution_settings_v0::handler(ctx, args)
  }

  pub fn update_state_v0(ctx: Context<UpdateStateV0>, args: UpdateStateArgsV0) -> Result<()> {
    update_state_v0::handler(ctx, args)
  }

  pub fn resolve_v0(ctx: Context<ResolveV0>) -> Result<()> {
    resolve_v0::handler(ctx)
  }
}
