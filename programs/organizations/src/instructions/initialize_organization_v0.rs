use crate::state::*;
use anchor_lang::prelude::*;

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
pub struct InitializeOrganizationArgsV0 {
  pub name: String,
  pub authority: Pubkey,
  pub default_vote_controller: Pubkey,
  pub default_state_controller: Pubkey,
  pub default_on_vote_hook: Pubkey,
  pub proposal_program: Pubkey,
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
