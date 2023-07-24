use crate::resolution_setting_seeds;
use crate::state::*;
use anchor_lang::prelude::*;
use proposal::ProposalConfigV0;
use proposal::ProposalState as CpiProposalState;
use proposal::ProposalV0;
use proposal::{
  cpi::{accounts::UpdateStateV0 as CpiUpdateStateV0, update_state_v0},
  UpdateStateArgsV0 as CpiUpdateStateArgsV0,
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, PartialEq)]
pub enum ProposalState {
  // Allow drafting proposal, in this state can add instructions and such to it
  #[default]
  Draft,
  Cancelled,
  Voting,
  /// Allow voting controller to set to a custom state,
  /// this allows for the implementation of more complex
  /// states like Vetoed, drafts, signing off, etc.
  /// This could have been an int, but then UIs would need to understand
  /// the calling contract to grab an enum from it. Rather just have something clean
  /// even though it takes a bit more space.
  Custom {
    name: String,
    bin: Vec<u8>,
  },
}

impl From<ProposalState> for CpiProposalState {
  fn from(value: ProposalState) -> Self {
    match value {
      ProposalState::Draft => CpiProposalState::Draft,
      ProposalState::Cancelled => CpiProposalState::Cancelled,
      ProposalState::Voting => CpiProposalState::Voting {
        start_ts: Clock::get().unwrap().unix_timestamp,
      },
      ProposalState::Custom { name, bin } => CpiProposalState::Custom { name, bin },
    }
  }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct UpdateStateArgsV0 {
  pub new_state: ProposalState,
}

/// Allow the owner to update ths state of the proposal when it is not resolved or voting.
#[derive(Accounts)]
pub struct UpdateStateV0<'info> {
  pub owner: Signer<'info>,
  #[account(
    mut,
    owner = proposal_program.key(),
    has_one = owner,
    has_one = proposal_config,
    constraint = match proposal.state {
      CpiProposalState::Voting { .. } => false,
      CpiProposalState::Resolved { .. } => false,
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
