use anchor_lang::prelude::*;
use proposal::{
  cpi::{accounts::UpdateStateV0 as CpiUpdateStateV0, update_state_v0},
  ProposalConfigV0, ProposalState, ProposalV0, UpdateStateArgsV0 as CpiUpdateStateArgsV0,
};

use crate::{resolution_setting_seeds, state::*};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct UpdateStateArgsV0 {
  pub new_state: ProposalState,
}

/// Allow the owner to update ths state of the proposal when it is not resolved or voting.
#[derive(Accounts)]
#[instruction(args: UpdateStateArgsV0)]
pub struct UpdateStateV0<'info> {
  pub owner: Signer<'info>,
  #[account(
    mut,
    owner = proposal_program.key(),
    has_one = owner,
    has_one = proposal_config,
    constraint = match proposal.state {
      // Voting can only go to state cancelled.
      ProposalState::Voting { .. } => args.new_state == ProposalState::Cancelled,
      ProposalState::Resolved { .. } => false,
      _ => true
    }
  )]
  pub proposal: Account<'info, ProposalV0>,
  #[account(
    has_one = state_controller
  )]
  pub proposal_config: Account<'info, ProposalConfigV0>,
  pub state_controller: Account<'info, ResolutionSettingsV0>,
  /// CHECK: Checked via `owner` on proposal
  pub proposal_program: AccountInfo<'info>,
}

pub fn handler(ctx: Context<UpdateStateV0>, args: UpdateStateArgsV0) -> Result<()> {
  update_state_v0(
    CpiContext::new_with_signer(
      ctx.accounts.proposal_program.to_account_info().clone(),
      CpiUpdateStateV0 {
        state_controller: ctx.accounts.state_controller.to_account_info().clone(),
        proposal: ctx.accounts.proposal.to_account_info().clone(),
        proposal_config: ctx.accounts.proposal_config.to_account_info().clone(),
      },
      &[resolution_setting_seeds!(ctx.accounts.state_controller)],
    ),
    CpiUpdateStateArgsV0 {
      new_state: args.new_state.into(),
    },
  )?;

  Ok(())
}
