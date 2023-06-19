use crate::resolution_setting_seeds;
use crate::state::*;
use anchor_lang::prelude::*;
use proposal::ProposalConfigV0;
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
    mut,
    owner = proposal_program.key(),
    has_one = proposal_config,
    constraint = proposal.to_account_info().is_signer,
    constraint = match proposal.state {
      ProposalState::Voting(_) => true,
      _ => false
    }
  )]
  pub proposal: Account<'info, ProposalV0>,
  #[account(
    has_one = state_controller,
    has_one = vote_controller,
  )]
  pub proposal_config: Account<'info, ProposalConfigV0>,
  /// CHECK: Checked via `owner` on proposal
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
          proposal_config: ctx.accounts.proposal_config.to_account_info().clone(),
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
