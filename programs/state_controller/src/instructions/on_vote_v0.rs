use crate::state::*;
use anchor_lang::prelude::*;
use proposal::ProposalConfigV0;
use proposal::ProposalState;
use proposal::ProposalV0;
use proposal::VoteArgsV0 as ProposalVoteArgsV0;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct VoteArgsV0 {
  pub choice: u16,
  pub weight: u128,
  /// This is a remove operation
  pub remove_vote: bool,
}

impl From<VoteArgsV0> for ProposalVoteArgsV0 {
  fn from(args: VoteArgsV0) -> Self {
    Self {
      choice: args.choice,
      weight: args.weight,
      remove_vote: args.remove_vote,
    }
  }
}

#[derive(Accounts)]
#[instruction(args: VoteArgsV0)]
pub struct OnVoteV0<'info> {
  /// CHECK: not used
  pub voter: AccountInfo<'info>,
  pub vote_controller: Signer<'info>,
  /// CHECK: Checked via cpi to the on vote hook
  #[account(mut)]
  pub state_controller: Account<'info, ResolutionSettingsV0>,
  #[account(
    // Short circuit anchor owner check since we don't care the program.
    // Allow for polymorphism
    owner = *proposal.to_account_info().owner,
    has_one = proposal_config,
    constraint = proposal.to_account_info().is_signer,
    constraint = match proposal.state {
      ProposalState::Voting { .. } => true,
      _ => false
    }
  )]
  pub proposal: Account<'info, ProposalV0>,
  #[account(
    has_one = state_controller,
    has_one = vote_controller,
  )]
  pub proposal_config: Account<'info, ProposalConfigV0>,
}

pub fn handler(ctx: Context<OnVoteV0>, _args: VoteArgsV0) -> Result<Option<Vec<u16>>> {
  let proposal = ctx.accounts.proposal.clone().into_inner();
  let resolution = ctx.accounts.state_controller.settings.resolution(&proposal);

  Ok(resolution)
}
