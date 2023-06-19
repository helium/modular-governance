use crate::state::*;
use anchor_lang::prelude::*;
use organization::state::OrganizationV0;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeOrganizationWalletArgsV0 {
  pub name: String,
  pub authority: Pubkey,
  pub proposal_config: Pubkey,
  pub index: u16,
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
      seeds = [b"organization_wallet", organization.key().as_ref(), &args.index.to_le_bytes()],
      bump
    )]
  pub organization_wallet: Box<Account<'info, OrganizationWalletV0>>,
  pub organization: Account<'info, OrganizationV0>,
  pub system_program: Program<'info, System>,
}

pub fn handler(
  ctx: Context<InitializeOrganizationWalletV0>,
  args: InitializeOrganizationWalletArgsV0,
) -> Result<()> {
  let (wallet, wallet_bump) = Pubkey::find_program_address(
    &[
      b"wallet",
      ctx.accounts.organization.key().as_ref(),
      &args.index.to_le_bytes(),
    ],
    &crate::id(),
  );
  ctx
    .accounts
    .organization_wallet
    .set_inner(OrganizationWalletV0 {
      name: args.name,
      organization: ctx.accounts.organization.key(),
      wallet,
      index: args.index,
      wallet_bump_seed: wallet_bump,
      proposal_config: args.proposal_config,
      bump_seed: ctx.bumps["organization_wallet"],
    });
  Ok(())
}
