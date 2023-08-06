use anchor_lang::prelude::*;

use crate::state::OrganizationV0;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct UpdateOrganizationArgsV0 {
  pub authority: Option<Pubkey>,
  pub default_proposal_config: Option<Pubkey>,
  pub proposal_program: Option<Pubkey>,
  pub uri: Option<String>,
}

#[derive(Accounts)]
#[instruction(args: UpdateOrganizationArgsV0)]
pub struct UpdateOrganizationV0<'info> {
  #[account(mut, has_one = authority)]
  pub organization: Box<Account<'info, OrganizationV0>>,
  pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateOrganizationV0>, args: UpdateOrganizationArgsV0) -> Result<()> {
  if let Some(authority) = args.authority {
    ctx.accounts.organization.authority = authority;
  }
  if let Some(default_proposal_config) = args.default_proposal_config {
    ctx.accounts.organization.default_proposal_config = default_proposal_config;
  }
  if let Some(proposal_program) = args.proposal_program {
    ctx.accounts.organization.proposal_program = proposal_program;
  }
  if let Some(uri) = args.uri {
    ctx.accounts.organization.uri = uri;
  }
  Ok(())
}
