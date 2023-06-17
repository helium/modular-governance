use crate::state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct UpdateStateArgsV0 {
  pub new_state: ProposalState,
}

#[derive(Accounts)]
#[instruction(args: UpdateStateArgsV0)]
pub struct UpdateStateV0<'info> {
  pub state_controller: Signer<'info>,
  #[account(
    has_one = proposal_config,
  )]
  pub proposal: Account<'info, ProposalV0>,
  #[account(
    has_one = state_controller
  )]
  pub proposal_config: Account<'info, ProposalConfigV0>,
}

pub fn handler(ctx: Context<UpdateStateV0>, args: UpdateStateArgsV0) -> Result<()> {
  ctx.accounts.proposal.state = args.new_state;

  Ok(())
}
