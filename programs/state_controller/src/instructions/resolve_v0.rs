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
pub struct ResolveV0<'info> {
  #[account(mut)]
  pub state_controller: Account<'info, ResolutionSettingsV0>,
  #[account(
    mut,
    owner = proposal_program.key(),
    has_one = proposal_config,
    constraint = matches!(proposal.state, ProposalState::Voting { .. })
  )]
  pub proposal: Account<'info, ProposalV0>,
  #[account(
    has_one = state_controller,
  )]
  pub proposal_config: Account<'info, ProposalConfigV0>,
  /// CHECK: Checked via `owner` on proposal
  pub proposal_program: AccountInfo<'info>,
}

pub fn handler(ctx: Context<ResolveV0>) -> Result<()> {
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
        new_state: ProposalState::Resolved {
          choices: resolution,
          end_ts: Clock::get()?.unix_timestamp,
        },
      },
    )?;
  }

  Ok(())
}
