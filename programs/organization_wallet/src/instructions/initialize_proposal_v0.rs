use crate::{organization_seeds, state::*};
use anchor_lang::prelude::*;
use proposal::{
  cpi::{
    accounts::InitializeProposalV0 as CpiInitializeProposal,
    initialize_proposal_v0 as cpi_initialize_proposal,
  },
  Choice as CpiChoice, InitializeProposalArgsV0 as CpiInitializeProposalArgs,
};

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Choice {
  /// Total vote weight behind this choice. u128 to support u64 tokens multiplied by a large multiplier (as in helium)
  pub weight: u128,
  #[max_len(200)]
  pub name: String,
  /// Any other data that you may want to put in here
  #[max_len(200)]
  pub uri: Option<String>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeProposalArgsV0 {
  /// Allow a custom seed for indexing
  pub seed: Vec<u8>,
  pub vote_controller: Option<Pubkey>,
  pub state_controller: Option<Pubkey>,
  pub on_vote_hook: Option<Pubkey>,
  pub name: String,
  pub uri: String,
  pub choices: Vec<Choice>,
  // Tags which can be used to filter proposals
  pub tags: Vec<String>,
}

#[derive(Accounts)]
#[instruction(args: InitializeProposalArgsV0)]
pub struct InitializeProposalV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  pub authority: Signer<'info>,
  #[account(
      mut,
      seeds = [
        b"proposal",
        organization.key().as_ref(),
        &organization.num_proposals.to_le_bytes()[..]
      ],
      seeds::program = organization.proposal_program,
      bump,
    )]
  /// CHECK: Checked via cpi
  pub proposal: AccountInfo<'info>,
  #[account(
      mut,
      has_one = proposal_program,
      has_one = authority
    )]
  pub organization: Box<Account<'info, OrganizationV0>>,
  pub proposal_program: Program<'info, System>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeProposalV0>, args: InitializeProposalArgsV0) -> Result<()> {
  cpi_initialize_proposal(
    CpiContext::new_with_signer(
      ctx.accounts.proposal_program.to_account_info(),
      CpiInitializeProposal {
        owner: ctx.accounts.organization.to_account_info(),
        proposal: ctx.accounts.proposal.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
      },
      &[organization_seeds!(ctx.accounts.organization)],
    ),
    CpiInitializeProposalArgs {
      seed: ctx
        .accounts
        .organization
        .num_proposals
        .to_le_bytes()
        .to_vec(),
      vote_controller: args
        .vote_controller
        .unwrap_or_else(|| ctx.accounts.organization.default_vote_controller),
      state_controller: args
        .vote_controller
        .unwrap_or_else(|| ctx.accounts.organization.default_state_controller),
      on_vote_hook: args
        .vote_controller
        .unwrap_or_else(|| ctx.accounts.organization.default_on_vote_hook),
      name: args.name,
      uri: args.uri,
      choices: args
        .choices
        .into_iter()
        .map(|c| CpiChoice {
          weight: c.weight,
          name: c.name,
          uri: c.uri,
        })
        .collect::<Vec<_>>(),
      tags: args.tags,
    },
  )?;
  Ok(())
}
