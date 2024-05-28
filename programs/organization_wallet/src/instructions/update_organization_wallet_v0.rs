use crate::error::ErrorCode;
use crate::state::OrganizationWalletV0;
use anchor_lang::prelude::*;
use organization::state::OrganizationV0;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct UpdateOrganizationWalletArgsV0 {
  pub name: Option<String>,
  pub proposal_configs: Option<Vec<Pubkey>>,
}

#[derive(Accounts)]
#[instruction(args: UpdateOrganizationWalletArgsV0)]
pub struct UpdateOrganizationWalletV0<'info> {
  /// CHECK: Payer
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    mut,
    constraint = organization_wallet.organization == organization.key() @ ErrorCode::InvalidOrganization,
  )]
  pub organization_wallet: Box<Account<'info, OrganizationWalletV0>>,
  #[account(
    has_one = authority
  )]
  pub organization: Account<'info, OrganizationV0>,
  pub authority: Signer<'info>,
}

pub fn handler(
  ctx: Context<UpdateOrganizationWalletV0>,
  args: UpdateOrganizationWalletArgsV0,
) -> Result<()> {
  if let Some(name) = args.name {
    ctx.accounts.organization_wallet.name = name;
  }
  if let Some(proposal_configs) = args.proposal_configs {
    ctx.accounts.organization_wallet.proposal_configs = proposal_configs;
  }
  Ok(())
}
