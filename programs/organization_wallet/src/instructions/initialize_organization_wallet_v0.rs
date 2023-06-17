use crate::state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeOrganizationWalletArgsV0 {
  pub name: String,
  pub authority: Pubkey,
  pub default_vote_controller: Pubkey,
  pub default_state_controller: Pubkey,
  pub default_on_vote_hook: Pubkey,
  pub proposal_program: Pubkey,
}

#[derive(Accounts)]
#[instruction(args: InitializeOrganizationWalletArgsV0)]
pub struct InitializeOrganizationWalletV0<'info> {
  /// CHECK: Payer
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
      init,
      payer = payer,
      space = 8 + 60 + OrganizationWalletV0::INIT_SPACE,
      seeds = [b"organization_wallet", args.name.as_bytes()],
      bump
    )]
  pub organization_wallet: Box<Account<'info, OrganizationV0>>,
  pub organization: Account<'info, OrganizationV0>,
  pub system_program: Program<'info, System>,
}

pub fn handler(
  ctx: Context<InitializeOrganizationV0>,
  args: InitializeOrganizationArgsV0,
) -> Result<()> {
  ctx.accounts.organization.set_inner(OrganizationV0 {
    name: args.name,
    authority: args.authority,
    bump_seed: ctx.bumps["organization"],
    num_proposals: 0,
    default_state_controller: args.default_state_controller,
    default_vote_controller: args.default_vote_controller,
    default_on_vote_hook: args.default_on_vote_hook,
    proposal_program: args.proposal_program,
  });
  Ok(())
}
