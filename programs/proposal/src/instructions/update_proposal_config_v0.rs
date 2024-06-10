use crate::state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct UpdateProposalConfigArgsV0 {
  pub vote_controller: Option<Pubkey>,
  pub state_controller: Option<Pubkey>,
  pub on_vote_hook: Option<Pubkey>,
  pub authority: Option<Pubkey>,
}

#[derive(Accounts)]
pub struct UpdateProposalConfigV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    mut,
    has_one = authority,
  )]
  pub proposal_config: Box<Account<'info, ProposalConfigV0>>,
  pub authority: Signer<'info>,
}

pub fn handler(
  ctx: Context<UpdateProposalConfigV0>,
  args: UpdateProposalConfigArgsV0,
) -> Result<()> {
  if let Some(vote_controller) = args.vote_controller {
    ctx.accounts.proposal_config.vote_controller = vote_controller;
  }
  if let Some(state_controller) = args.state_controller {
    ctx.accounts.proposal_config.state_controller = state_controller;
  }
  if let Some(on_vote_hook) = args.on_vote_hook {
    ctx.accounts.proposal_config.on_vote_hook = on_vote_hook;
  }
  if let Some(authority) = args.authority {
    ctx.accounts.proposal_config.authority = authority;
  }

  Ok(())
}
