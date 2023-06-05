use crate::state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ResolveArgsV0 {
    pub resolution: Vec<u16>,
}

#[derive(Accounts)]
#[instruction(args: ResolveArgsV0)]
pub struct ResolveV0<'info> {
    pub resolution_controller: Signer<'info>,
    #[account(
    has_one = resolution_controller,
  )]
    pub proposal: Account<'info, ProposalV0>,
}

pub fn handler(ctx: Context<ResolveV0>, args: ResolveArgsV0) -> Result<()> {
    ctx.accounts.proposal.state = ProposalState::Resolved(args.resolution);

    Ok(())
}
