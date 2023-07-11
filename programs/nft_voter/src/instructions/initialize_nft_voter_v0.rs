use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeNftVoterArgsV0 {
  pub name: String,
  pub authority: Pubkey,
}

#[derive(Accounts)]
#[instruction(args: InitializeNftVoterArgsV0)]
pub struct InitializeNftVoterV0<'info> {
  /// CHECK: Payer
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    init,
    payer = payer,
    space = 8 + 60 + std::mem::size_of::<NftVoterV0>(),
    seeds = [b"nft_voter", args.name.as_bytes()],
    bump
  )]
  pub nft_voter: Box<Account<'info, NftVoterV0>>,
  pub collection: Box<Account<'info, Mint>>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeNftVoterV0>, args: InitializeNftVoterArgsV0) -> Result<()> {
  ctx.accounts.nft_voter.set_inner(NftVoterV0 {
    bump_seed: ctx.bumps["nft_voter"],
    name: args.name,
    authority: args.authority,
    collection: ctx.accounts.collection.key(),
  });

  Ok(())
}
