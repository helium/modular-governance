use crate::{proposal_seeds, state::*};
use anchor_lang::prelude::*;
use vote_hook_interface::cpi::{accounts::OnVoteV0, on_vote_v0};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct VoteArgsV0 {
    pub choice: u16,
    pub weight: u128,
    /// This is a remove operation
    pub remove_vote: bool,
}

#[derive(Accounts)]
#[instruction(args: VoteArgsV0)]
pub struct VoteV0<'info> {
    pub vote_controller: Signer<'info>,
    /// CHECK: Checked via cpi to the on vote hook, and has_ones
    #[account(mut)]
    pub state_controller: AccountInfo<'info>,
    #[account(
    has_one = on_vote_hook,
    has_one = state_controller,
    has_one = vote_controller
  )]
    pub proposal: Account<'info, ProposalV0>,
    /// CHECK: Checked via has_one
    pub on_vote_hook: AccountInfo<'info>,
    /// CHECK: Checked via constraint
    #[account(
    constraint = *proposal.to_account_info().owner == proposal_program.key()
  )]
    pub proposal_program: AccountInfo<'info>,
}

pub fn handler(ctx: Context<VoteV0>, args: VoteArgsV0) -> Result<()> {
    if args.remove_vote {
        ctx.accounts.proposal.remove_vote(Vote {
            choice: args.choice,
            weight: args.weight,
        });
    } else {
        ctx.accounts.proposal.vote(Vote {
            choice: args.choice,
            weight: args.weight,
        });
    }

    on_vote_v0(
        CpiContext::new_with_signer(
            ctx.accounts.on_vote_hook.clone(),
            OnVoteV0 {
                vote_controller: ctx.accounts.vote_controller.to_account_info().clone(),
                state_controller: ctx.accounts.state_controller.clone(),
                proposal: ctx.accounts.proposal.to_account_info().clone(),
                proposal_program: ctx.accounts.proposal_program.to_account_info().clone(),
            },
            &[proposal_seeds!(ctx.accounts.proposal)],
        ),
        vote_hook_interface::VoteArgsV0 {
            choice: args.choice,
            weight: args.weight,
            remove_vote: args.remove_vote,
        },
    )?;
    Ok(())
}
