use crate::state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeOrganizationArgsV0 {
  pub name: String,
  pub authority: Pubkey,
  pub default_proposal_config: Pubkey,
  pub proposal_program: Pubkey,
  pub uri: String,
}

#[derive(Accounts)]
#[instruction(args: InitializeOrganizationArgsV0)]
pub struct InitializeOrganizationV0<'info> {
  /// CHECK: Payer
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
      init,
      payer = payer,
      space = 8 + 60 + OrganizationV0::INIT_SPACE,
      seeds = [b"organization", args.name.as_bytes()],
      bump
    )]
  pub organization: Box<Account<'info, OrganizationV0>>,
  pub system_program: Program<'info, System>,
}

pub fn handler(
  ctx: Context<InitializeOrganizationV0>,
  args: InitializeOrganizationArgsV0,
) -> Result<()> {
  require_gt!(32, args.name.len());
  require_gt!(200, args.uri.len());

  ctx.accounts.organization.set_inner(OrganizationV0 {
    name: args.name,
    authority: args.authority,
    bump_seed: ctx.bumps["organization"],
    num_proposals: 0,
    default_proposal_config: args.default_proposal_config,
    proposal_program: args.proposal_program,
    uri: args.uri,
  });
  Ok(())
}
