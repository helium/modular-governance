use crate::state::*;
use anchor_lang::prelude::*;
use organization::state::OrganizationV0;
use proposal::{ProposalState, ProposalV0};

#[derive(Accounts)]
pub struct InitializeWalletProposalV0<'info> {
  /// CHECK: Payer
  #[account(mut)]
  pub payer: Signer<'info>,
  pub organization_wallet: Box<Account<'info, OrganizationWalletV0>>,
  pub authority: Signer<'info>,
  #[account(has_one = authority)]
  pub owner: Account<'info, OrganizationV0>,
  #[account(
    constraint = proposal.state == ProposalState::Draft,
    constraint = proposal.proposal_config == organization_wallet.proposal_config,
    has_one = owner
  )]
  pub proposal: Box<Account<'info, ProposalV0>>,
  #[account(
    init,
    payer = payer,
    space = 8 + 60 + std::mem::size_of::<WalletProposalV0>(),
    seeds = [b"wallet_proposal", organization_wallet.key().as_ref(), proposal.key().as_ref()],
    bump
  )]
  pub wallet_proposal: Box<Account<'info, WalletProposalV0>>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeWalletProposalV0>) -> Result<()> {
  ctx.accounts.wallet_proposal.set_inner(WalletProposalV0 {
    organization_wallet: ctx.accounts.organization_wallet.key(),
    proposal: ctx.accounts.proposal.key(),
    bump_seed: ctx.bumps["wallet_proposal"],
    choice_transactions: vec![],
  });
  Ok(())
}
