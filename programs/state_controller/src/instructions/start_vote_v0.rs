use crate::resolution_setting_seeds;
use crate::state::*;
use anchor_lang::prelude::*;
use proposal::ProposalConfigV0;
use proposal::ProposalState;
use proposal::ProposalV0;
use proposal::{
  cpi::{accounts::UpdateStateV0, update_state_v0},
  UpdateStateArgsV0,
};

#[derive(Accounts)]
pub struct StartVoteV0<'info> {
  pub owner: Signer<'info>,
  #[account(
    mut,
    owner = proposal_program.key(),
    has_one = owner,
    has_one = proposal_config,
    constraint = proposal.state == ProposalState::Draft
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

pub fn handler(ctx: Context<StartVoteV0>) -> Result<()> {
  update_state_v0(
    CpiContext::new_with_signer(
      ctx.accounts.proposal_program.to_account_info().clone(),
      UpdateStateV0 {
        state_controller: ctx.accounts.state_controller.to_account_info().clone(),
        proposal: ctx.accounts.proposal.to_account_info().clone(),
        proposal_config: ctx.accounts.proposal_config.to_account_info().clone(),
      },
      &[resolution_setting_seeds!(ctx.accounts.state_controller)],
    ),
    UpdateStateArgsV0 {
      new_state: ProposalState::Voting(Clock::get()?.unix_timestamp),
    },
  )?;

  Ok(())
}
