use crate::{organization_seeds, state::*};
use anchor_lang::prelude::*;
use proposal::{
  cpi::{
    accounts::InitializeProposalV0 as CpiInitializeProposal,
    initialize_proposal_v0 as cpi_initialize_proposal,
  },
  ChoiceArg as CpiChoice, InitializeProposalArgsV0 as CpiInitializeProposalArgs,
};

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ChoiceArg {
  #[max_len(200)]
  pub name: String,
  /// Any other data that you may want to put in here
  #[max_len(200)]
  pub uri: Option<String>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeProposalArgsV0 {
  pub name: String,
  pub uri: String,
  pub max_choices_per_voter: u16,
  pub choices: Vec<ChoiceArg>,
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
  /// CHECK: Checked via cpi
  pub proposal_config: AccountInfo<'info>,
  #[account(
      mut,
      has_one = proposal_program,
      has_one = authority
    )]
  pub organization: Box<Account<'info, OrganizationV0>>,
  /// CHECK: Checked via address constraint
  #[account(
    address = organization.proposal_program
  )]
  pub proposal_program: UncheckedAccount<'info>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeProposalV0>, args: InitializeProposalArgsV0) -> Result<()> {
  cpi_initialize_proposal(
    CpiContext::new_with_signer(
      ctx.accounts.proposal_program.to_account_info(),
      CpiInitializeProposal {
        owner: ctx.accounts.organization.to_account_info(),
        proposal: ctx.accounts.proposal.to_account_info(),
        proposal_config: ctx.accounts.proposal_config.to_account_info(),
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
      name: args.name,
      max_choices_per_voter: args.max_choices_per_voter,
      uri: args.uri,
      choices: args
        .choices
        .into_iter()
        .map(|c| CpiChoice {
          name: c.name,
          uri: c.uri,
        })
        .collect::<Vec<_>>(),
      tags: args.tags,
    },
  )?;
  Ok(())
}
