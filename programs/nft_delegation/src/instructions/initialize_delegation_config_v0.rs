use crate::state::DelegationConfigV0;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeDelegationConfigArgsV0 {
  pub name: String,
  pub max_delegation_time: i64,
}

#[derive(Accounts)]
#[instruction(args: InitializeDelegationConfigArgsV0)]
pub struct InitializeDelegationConfigV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  /// CHECK: The authority of this config
  pub authority: AccountInfo<'info>,
  #[account(
    init,
    payer = payer,
    space = 60 + DelegationConfigV0::INIT_SPACE,
    seeds = [b"delegation_config".as_ref(), args.name.as_bytes()],
    bump
  )]
  pub delegation_config: Account<'info, DelegationConfigV0>,
  pub system_program: Program<'info, System>,
}

pub fn handler(
  ctx: Context<InitializeDelegationConfigV0>,
  args: InitializeDelegationConfigArgsV0,
) -> Result<()> {
  ctx
    .accounts
    .delegation_config
    .set_inner(DelegationConfigV0 {
      authority: ctx.accounts.authority.key(),
      name: args.name,
      max_delegation_time: args.max_delegation_time,
    });

  Ok(())
}
