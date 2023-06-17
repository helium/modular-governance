use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("org9nsbSiTCJzeApoS2B3uwjM2gbQH48QbUUrhAAjzG");

#[program]
pub mod organizations {
  use super::*;

  pub fn initialize_organization_v0(
    ctx: Context<InitializeOrganizationV0>,
    args: InitializeOrganizationArgsV0,
  ) -> Result<()> {
    initialize_organization_v0::handler(ctx, args)
  }

  pub fn initialize_proposal_v0(
    ctx: Context<InitializeProposalV0>,
    args: InitializeProposalArgsV0,
  ) -> Result<()> {
    initialize_proposal_v0::handler(ctx, args)
  }
}
