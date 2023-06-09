use crate::resolution_setting_seeds;
use crate::state::*;
use anchor_lang::prelude::*;
use proposal::ProposalState;
use proposal::ProposalV0;
use proposal::VoteArgsV0;
use proposal::{
  cpi::{accounts::UpdateStateV0, update_state_v0},
  UpdateStateArgsV0,
};

#[derive(Accounts)]
#[instruction(args: VoteArgsV0)]
pub struct OnVoteV0<'info> {
  pub vote_controller: Signer<'info>,
  /// CHECK: Checked via cpi to the on vote hook
  #[account(mut)]
  pub state_controller: Account<'info, ResolutionSettingsV0>,
  #[account(
    has_one = state_controller,
    has_one = vote_controller,
    constraint = proposal.to_account_info().is_signer,
    constraint = proposal.state == ProposalState::Voting
  )]
  pub proposal: Account<'info, ProposalV0>,
  /// CHECK: Checked via constraint
  #[account(
      constraint = *proposal.to_account_info().owner == proposal_program.key()
    )]
  pub proposal_program: AccountInfo<'info>,
}

pub fn handler(ctx: Context<OnVoteV0>, _args: VoteArgsV0) -> Result<()> {
  let proposal = ctx.accounts.proposal.clone().into_inner();
  if let Some(resolution) = ctx.accounts.state_controller.settings.resolution(&proposal) {
    update_state_v0(
      CpiContext::new_with_signer(
        ctx.accounts.proposal_program.to_account_info().clone(),
        UpdateStateV0 {
          state_controller: ctx.accounts.state_controller.to_account_info().clone(),
          proposal: ctx.accounts.proposal.to_account_info().clone(),
        },
        &[resolution_setting_seeds!(ctx.accounts.state_controller)],
      ),
      UpdateStateArgsV0 {
        new_state: ProposalState::Resolved(resolution),
      },
    )?;
  }

  Ok(())
}
