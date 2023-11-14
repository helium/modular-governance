use crate::{error::ErrorCode, metaplex::MetadataAccount, VoteArgsV0};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use nft_delegation::state::DelegationV0;
use proposal::{ProposalConfigV0, ProposalV0};

use crate::{nft_voter_seeds, state::*};

#[derive(Accounts)]
pub struct DelegatedVoteV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 8 + 60 + std::mem::size_of::<VoteMarkerV0>(),
    seeds = [b"marker", nft_voter.key().as_ref(), mint.key().as_ref(), proposal.key().as_ref()],
    bump
  )]
  pub marker: Box<Account<'info, VoteMarkerV0>>,
  #[account(
    has_one = owner,
    // only the current or earlier delegates can change vote. Or if proposal not set, this was an `init` for the marker
    constraint = delegation.index <= marker.delegation_index || marker.proposal == Pubkey::default()
  )]
  pub delegation: Box<Account<'info, DelegationV0>>,
  pub nft_voter: Box<Account<'info, NftVoterV0>>,
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

pub fn handler(ctx: Context<DelegatedVoteV0>, args: VoteArgsV0) -> Result<()> {
  let marker = &mut ctx.accounts.marker;
  if marker.rent_refund == Pubkey::default() {
    marker.rent_refund = ctx.accounts.payer.key();
  }
  marker.proposal = ctx.accounts.proposal.key();
  marker.bump_seed = ctx.bumps["marker"];
  marker.voter = ctx.accounts.owner.key();
  marker.nft_voter = ctx.accounts.nft_voter.key();
  marker.mint = ctx.accounts.mint.key();
  marker.delegation_index = ctx.accounts.delegation.index;

  // Don't allow voting for the same choice twice.
  require!(
    marker.choices.iter().all(|choice| *choice != args.choice),
    ErrorCode::AlreadyVoted
  );
  require_gt!(
    ctx.accounts.proposal.max_choices_per_voter,
    marker.choices.len() as u16,
    ErrorCode::MaxChoicesExceeded
  );

  marker.choices.push(args.choice);

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
      remove_vote: false,
      choice: args.choice,
      weight: 1_u128,
    },
  )?;

  Ok(())
}
