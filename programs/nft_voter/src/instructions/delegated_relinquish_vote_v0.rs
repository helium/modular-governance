use crate::{error::ErrorCode, metaplex::MetadataAccount, RelinquishVoteArgsV0};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use nft_delegation::state::DelegationV0;
use proposal::{ProposalConfigV0, ProposalV0};

use crate::{nft_voter_seeds, state::*};

#[derive(Accounts)]
pub struct DelegatedRelinquishVoteV0<'info> {
  /// CHECK: You're getting sol why do you care?
  /// Account to receive sol rent_refund if marker is closed
  #[account(mut)]
  pub rent_refund: AccountInfo<'info>,
  #[account(
    mut,
    seeds = [b"marker", nft_voter.key().as_ref(), mint.key().as_ref(), proposal.key().as_ref()],
    bump = marker.bump_seed,
    has_one = nft_voter,
    has_one = rent_refund
  )]
  pub marker: Account<'info, VoteMarkerV0>,
  pub nft_voter: Account<'info, NftVoterV0>,
  pub owner: Signer<'info>,
  pub mint: Box<Account<'info, Mint>>,
  #[account(
    seeds = ["metadata".as_bytes(), MetadataAccount::owner().as_ref(), mint.key().as_ref()],
    seeds::program = MetadataAccount::owner(),
    bump,
    constraint = metadata.collection.as_ref().map(|col| col.verified && col.key == nft_voter.collection).unwrap_or_else(|| false)
  )]
  pub metadata: Box<Account<'info, MetadataAccount>>,
  #[account(
    has_one = owner,
    constraint = delegation.index <= marker.delegation_index
  )]
  pub delegation: Box<Account<'info, DelegationV0>>,
  #[account(
    mut,
    has_one = proposal_config,
    owner = proposal_program.key(),
  )]
  pub proposal: Account<'info, ProposalV0>,
  #[account(
    has_one = on_vote_hook,
    has_one = state_controller,
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

pub fn handler(ctx: Context<DelegatedRelinquishVoteV0>, args: RelinquishVoteArgsV0) -> Result<()> {
  let marker = &mut ctx.accounts.marker;
  marker.proposal = ctx.accounts.proposal.key();
  marker.voter = ctx.accounts.owner.key();

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
        voter: ctx.accounts.owner.to_account_info(),
        vote_controller: ctx.accounts.nft_voter.to_account_info(),
        state_controller: ctx.accounts.state_controller.to_account_info(),
        proposal_config: ctx.accounts.proposal_config.to_account_info(),
        proposal: ctx.accounts.proposal.to_account_info(),
        on_vote_hook: ctx.accounts.on_vote_hook.to_account_info(),
      },
      &[nft_voter_seeds!(ctx.accounts.nft_voter)],
    ),
    proposal::VoteArgsV0 {
      remove_vote: true,
      choice: args.choice,
      weight: 1_u128,
    },
  )?;

  if marker.choices.len() == 0 {
    marker.close(ctx.accounts.rent_refund.to_account_info())?;
  }

  Ok(())
}
