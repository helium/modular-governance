use crate::metaplex::{
  create_master_edition_v3, create_metadata_accounts_v3, CollectionDetails, CreateMasterEditionV3,
  CreateMetadataAccountsV3, DataV2, Metadata,
};
use crate::{state::*, token_voter_seeds};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeTokenVoterArgsV0 {
  pub name: String,
  pub authority: Pubkey,
  pub collection_uri: String,
}

#[derive(Accounts)]
#[instruction(args: InitializeTokenVoterArgsV0)]
pub struct InitializeTokenVoterV0<'info> {
  /// CHECK: Payer
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    init,
    payer = payer,
    space = 8 + 60 + std::mem::size_of::<TokenVoterV0>(),
    seeds = [b"token_voter", args.name.as_bytes()],
    bump
  )]
  pub token_voter: Box<Account<'info, TokenVoterV0>>,
  #[account(
    init,
    payer = payer,
    mint::decimals = 0,
    mint::authority = token_voter,
    mint::freeze_authority = token_voter,
    seeds = ["collection".as_bytes(), token_voter.key().as_ref()],
    bump
  )]
  pub collection: Box<Account<'info, Mint>>,
  /// CHECK: Handled by cpi
  #[account(
    mut,
    seeds = ["metadata".as_bytes(), token_metadata_program.key().as_ref(), collection.key().as_ref()],
    seeds::program = token_metadata_program.key(),
    bump,
  )]
  pub metadata: UncheckedAccount<'info>,
  /// CHECK: Handled by cpi
  #[account(
    mut,
    seeds = ["metadata".as_bytes(), token_metadata_program.key().as_ref(), collection.key().as_ref(), "edition".as_bytes()],
    seeds::program = token_metadata_program.key(),
    bump,
  )]
  pub master_edition: UncheckedAccount<'info>,
  #[account(
    init_if_needed,
    payer = payer,
    associated_token::mint = collection,
    associated_token::authority = token_voter,
  )]
  pub token_account: Box<Account<'info, TokenAccount>>,
  pub mint: Box<Account<'info, Mint>>,
  pub rent: Sysvar<'info, Rent>,
  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_metadata_program: Program<'info, Metadata>,
  pub system_program: Program<'info, System>,
}

impl<'info> InitializeTokenVoterV0<'info> {
  fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
    let cpi_accounts = MintTo {
      mint: self.collection.to_account_info(),
      to: self.token_account.to_account_info(),
      authority: self.token_voter.to_account_info(),
    };
    CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
  }
}

pub fn handler(
  ctx: Context<InitializeTokenVoterV0>,
  args: InitializeTokenVoterArgsV0,
) -> Result<()> {
  ctx.accounts.token_voter.set_inner(TokenVoterV0 {
    bump_seed: ctx.bumps["token_voter"],
    name: args.name,
    authority: args.authority,
    deposit_mint: ctx.accounts.mint.key(),
    collection: ctx.accounts.collection.key(),
  });

  let signer_seeds: &[&[&[u8]]] = &[token_voter_seeds!(ctx.accounts.token_voter)];

  token::mint_to(ctx.accounts.mint_ctx().with_signer(signer_seeds), 1)?;

  create_metadata_accounts_v3(
    CpiContext::new_with_signer(
      ctx
        .accounts
        .token_metadata_program
        .to_account_info()
        .clone(),
      CreateMetadataAccountsV3 {
        metadata: ctx.accounts.metadata.to_account_info().clone(),
        mint: ctx.accounts.collection.to_account_info().clone(),
        mint_authority: ctx.accounts.token_voter.to_account_info().clone(),
        payer: ctx.accounts.payer.to_account_info().clone(),
        update_authority: ctx.accounts.token_voter.to_account_info().clone(),
        system_program: ctx.accounts.system_program.to_account_info().clone(),
        rent: ctx.accounts.rent.to_account_info().clone(),
      },
      signer_seeds,
    ),
    DataV2 {
      name: String::from("Token Voter Receipts"),
      symbol: String::from("TVR"),
      uri: args.collection_uri,
      seller_fee_basis_points: 0,
      creators: None,
      collection: None,
      uses: None,
    },
    true,
    true,
    Some(CollectionDetails::V1 { size: 0 }),
  )?;

  create_master_edition_v3(
    CpiContext::new_with_signer(
      ctx
        .accounts
        .token_metadata_program
        .to_account_info()
        .clone(),
      CreateMasterEditionV3 {
        edition: ctx.accounts.master_edition.to_account_info().clone(),
        mint: ctx.accounts.collection.to_account_info().clone(),
        update_authority: ctx.accounts.token_voter.to_account_info().clone(),
        mint_authority: ctx.accounts.token_voter.to_account_info().clone(),
        metadata: ctx.accounts.metadata.to_account_info().clone(),
        payer: ctx.accounts.payer.to_account_info().clone(),
        token_program: ctx.accounts.token_program.to_account_info().clone(),
        system_program: ctx.accounts.system_program.to_account_info().clone(),
        rent: ctx.accounts.rent.to_account_info().clone(),
      },
      signer_seeds,
    ),
    Some(0),
  )?;

  Ok(())
}
