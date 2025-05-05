use anchor_lang::prelude::*;

use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct UpdateProposalArgsV0 {
  pub name: Option<String>,
  pub uri: Option<String>,
  pub tags: Option<Vec<String>>,
}

#[derive(Accounts)]
pub struct UpdateProposalV0<'info> {
  #[account(
    mut,
    has_one = owner,
    constraint = match proposal.state {
      // Only allow updating name, uri, and tags if proposal has no votes
      ProposalState::Voting { .. } => proposal.choices.iter().all(|choice| choice.weight == 0),
      ProposalState::Resolved { .. } => false,
      _ => true
    }
  )]
  pub proposal: Box<Account<'info, ProposalV0>>,
  pub owner: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateProposalV0>, args: UpdateProposalArgsV0) -> Result<()> {
  if let Some(name) = args.name {
    ctx.accounts.proposal.name = name;
  }
  if let Some(uri) = args.uri {
    ctx.accounts.proposal.uri = uri;
  }
  if let Some(tags) = args.tags {
    ctx.accounts.proposal.tags = tags;
  }

  Ok(())
}
