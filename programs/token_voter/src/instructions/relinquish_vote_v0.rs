use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use proposal::{ProposalConfigV0, ProposalV0};

use crate::{state::*, token_voter_seeds};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct RelinquishVoteArgsV0 {
  pub choice: u16,
}

#[derive(Accounts)]
pub struct RelinquishVoteV0<'info> {
  /// CHECK: You're getting sol why do you care?
  /// Account to receive sol refund if marker is closed
  #[account(mut)]
  pub refund: AccountInfo<'info>,
  #[account(
    mut,
    seeds = [b"marker", proposal.key().as_ref()],
    bump = marker.bump_seed,
  )]
  pub marker: Account<'info, VoteMarkerV0>,
  pub vote_controller: Account<'info, TokenVoterV0>,
  pub voter: Signer<'info>,
  #[account(
    has_one = mint
  )]
  pub receipt: Box<Account<'info, ReceiptV0>>,
  pub mint: Box<Account<'info, Mint>>,
  #[account(
    associated_token::authority = voter,
    associated_token::mint = mint,
    constraint = token_account.amount == 1,
  )]
  pub token_account: Box<Account<'info, TokenAccount>>,
  #[account(
    mut,
    has_one = proposal_config,
    owner = proposal_program.key(),
  )]
  pub proposal: Account<'info, ProposalV0>,
  #[account(
    has_one = on_vote_hook,
    has_one = state_controller,
    has_one = vote_controller,
    owner = proposal_program.key()
  )]
  pub proposal_config: Account<'info, ProposalConfigV0>,
  /// CHECK: Checked via cpi
  #[account(mut)]
  pub state_controller: AccountInfo<'info>,
  /// CHECK: Checked via has_one
  pub on_vote_hook: AccountInfo<'info>,
  /// CHECK: Checked via constraint
  #[account(
    constraint = *proposal.to_account_info().owner == proposal_program.key()
  )]
  pub proposal_program: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RelinquishVoteV0>, args: RelinquishVoteArgsV0) -> Result<()> {
  let marker = &mut ctx.accounts.marker;
  marker.proposal = ctx.accounts.proposal.key();

  require!(
    marker.choices.iter().any(|choice| *choice == args.choice),
    ErrorCode::NoVoteForThisChoice
  );

  marker.choices = marker
    .choices
    .clone()
    .into_iter()
    .filter(|c| *c != args.choice)
    .collect::<Vec<_>>();

  proposal::cpi::vote_v0(
    CpiContext::new_with_signer(
      ctx.accounts.proposal_program.to_account_info(),
      proposal::cpi::accounts::VoteV0 {
        vote_controller: ctx.accounts.vote_controller.to_account_info(),
        voter: ctx.accounts.voter.to_account_info(),
        state_controller: ctx.accounts.state_controller.to_account_info(),
        proposal_config: ctx.accounts.proposal_config.to_account_info(),
        proposal: ctx.accounts.proposal.to_account_info(),
        on_vote_hook: ctx.accounts.on_vote_hook.to_account_info(),
      },
      &[token_voter_seeds!(ctx.accounts.vote_controller)],
    ),
    proposal::VoteArgsV0 {
      remove_vote: true,
      choice: args.choice,
      weight: u128::from(ctx.accounts.receipt.amount),
    },
  )?;

  if marker.choices.len() == 0 {
    marker.close(ctx.accounts.refund.to_account_info())?;
  }

  Ok(())
}
