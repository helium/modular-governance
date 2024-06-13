use anchor_lang::prelude::*;

declare_id!("propFYxqmVcufMhk5esNMrexq2ogHbbC2kP9PU1qxKs");

pub mod errors;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

#[program]
pub mod proposal {
  use super::*;

  pub fn initialize_proposal_v0(
    ctx: Context<InitializeProposalV0>,
    args: InitializeProposalArgsV0,
  ) -> Result<()> {
    initialize_proposal_v0::handler(ctx, args)
  }

  pub fn initialize_proposal_config_v0(
    ctx: Context<InitializeProposalConfigV0>,
    args: InitializeProposalConfigArgsV0,
  ) -> Result<()> {
    initialize_proposal_config_v0::handler(ctx, args)
  }

  pub fn vote_v0(ctx: Context<VoteV0>, args: VoteArgsV0) -> Result<()> {
    vote_v0::handler(ctx, args)
  }

  pub fn update_state_v0(ctx: Context<UpdateStateV0>, args: UpdateStateArgsV0) -> Result<()> {
    update_state_v0::handler(ctx, args)
  }

  pub fn update_proposal_config_v0(
    ctx: Context<UpdateProposalConfigV0>,
    args: UpdateProposalConfigArgsV0,
  ) -> Result<()> {
    update_proposal_config_v0::handler(ctx, args)
  }
}

#[derive(Accounts)]
pub struct Initialize {}
