use crate::metaplex::{
  create_master_edition_v3, create_metadata_accounts_v3, verify_sized_collection_item, Collection,
  CreateMasterEditionV3, CreateMetadataAccountsV3, DataV2, Metadata, VerifySizedCollectionItem,
};
use crate::receipt_seeds;
use crate::state::*;
use crate::token_voter_seeds;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Transfer};
use anchor_spl::token::{Mint, MintTo, Token, TokenAccount};
use std::mem::size_of;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct DepositArgsV0 {
  pub amount: u64,
  pub metadata_uri: String,
}

#[derive(Accounts)]
pub struct DepositV0<'info> {
  #[account(
    has_one = collection,
    has_one = deposit_mint,
  )]
  pub token_voter: Box<Account<'info, TokenVoterV0>>,
  pub collection: Box<Account<'info, Mint>>,
  /// CHECK: Handled by cpi
  #[account(
    mut,
    seeds = ["metadata".as_bytes(), token_metadata_program.key().as_ref(), collection.key().as_ref()],
    seeds::program = token_metadata_program.key(),
    bump,
  )]
  pub collection_metadata: UncheckedAccount<'info>,
  /// CHECK: Handled By cpi account
  #[account(
    mut,
    seeds = ["metadata".as_bytes(), token_metadata_program.key().as_ref(), collection.key().as_ref(), "edition".as_bytes()],
    seeds::program = token_metadata_program.key(),
    bump,
  )]
  pub collection_master_edition: UncheckedAccount<'info>,

  // checking the PDA address it just an extra precaution,
  // the other constraints must be exhaustive
  #[account(
    init,
    payer = payer,
    seeds = [b"receipt".as_ref(), mint.key().as_ref()],
    bump,
    space = 8 + size_of::<ReceiptV0>() + 60,
  )]
  pub receipt: Box<Account<'info, ReceiptV0>>,
  #[account(
    mut,
    constraint = mint.supply == 0,
    mint::decimals = 0,
    mint::authority = receipt,
    mint::freeze_authority = receipt,
  )]
  pub mint: Box<Account<'info, Mint>>,

  #[account(
    mut,
    seeds = ["metadata".as_bytes(), token_metadata_program.key().as_ref(), mint.key().as_ref()],
    seeds::program = token_metadata_program.key(),
    bump,
  )]
  /// CHECK: Checked by cpi
  pub metadata: UncheckedAccount<'info>,
  /// CHECK: Handled by cpi
  #[account(
    mut,
    seeds = ["metadata".as_bytes(), token_metadata_program.key().as_ref(), mint.key().as_ref(), "edition".as_bytes()],
    seeds::program = token_metadata_program.key(),
    bump,
  )]
  pub master_edition: UncheckedAccount<'info>,
  #[account(
    init_if_needed,
    payer = payer,
    associated_token::mint = mint,
    associated_token::authority = recipient,
  )]
  pub receipt_token_account: Box<Account<'info, TokenAccount>>,

  /// CHECK: needed for token account init
  pub recipient: UncheckedAccount<'info>,
  #[account(
    init_if_needed,
    associated_token::authority = receipt,
    associated_token::mint = deposit_mint,
    payer = payer
  )]
  pub vault: Box<Account<'info, TokenAccount>>,
  #[account(
    init_if_needed,
    associated_token::authority = payer,
    associated_token::mint = deposit_mint,
    payer = payer
  )]
  pub token_account: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub payer: Signer<'info>,

  pub deposit_mint: Box<Account<'info, Mint>>,

  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_metadata_program: Program<'info, Metadata>,
  pub rent: Sysvar<'info, Rent>,
}

impl<'info> DepositV0<'info> {
  fn mint_nft_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
    let cpi_accounts = MintTo {
      mint: self.mint.to_account_info(),
      to: self.receipt_token_account.to_account_info(),
      authority: self.receipt.to_account_info(),
    };
    CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
  }

  fn deposit_transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
    let cpi_accounts = Transfer {
      from: self.token_account.to_account_info(),
      to: self.vault.to_account_info(),
      authority: self.payer.to_account_info(),
    };
    CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
  }
}

pub fn handler(ctx: Context<DepositV0>, args: DepositArgsV0) -> Result<()> {
  ctx.accounts.receipt.set_inner(ReceiptV0 {
    token_voter: ctx.accounts.token_voter.key(),
    mint: ctx.accounts.mint.key(),
    bump_seed: ctx.bumps["receipt"],
    amount: args.amount,
    num_active_votes: 0,
  });

  let signer_seeds: &[&[&[u8]]] = &[receipt_seeds!(ctx.accounts.receipt)];

  token::mint_to(ctx.accounts.mint_nft_ctx().with_signer(signer_seeds), 1)?;
  token::transfer(ctx.accounts.deposit_transfer_ctx(), args.amount)?;

  create_metadata_accounts_v3(
    CpiContext::new_with_signer(
      ctx
        .accounts
        .token_metadata_program
        .to_account_info()
        .clone(),
      CreateMetadataAccountsV3 {
        metadata: ctx.accounts.metadata.to_account_info().clone(),
        mint: ctx.accounts.mint.to_account_info().clone(),
        mint_authority: ctx.accounts.receipt.to_account_info().clone(),
        payer: ctx.accounts.payer.to_account_info().clone(),
        update_authority: ctx.accounts.receipt.to_account_info().clone(),
        system_program: ctx.accounts.system_program.to_account_info().clone(),
        rent: ctx.accounts.rent.to_account_info().clone(),
      },
      signer_seeds,
    ),
    DataV2 {
      name: String::from("Token Voter Receipt"),
      symbol: String::from("TVR"),
      uri: args.metadata_uri,
      seller_fee_basis_points: 0,
      creators: None,
      collection: Some(Collection {
        key: ctx.accounts.token_voter.collection.key(),
        verified: false, // Verified in cpi
      }),
      uses: None,
    },
    true,
    true,
    None,
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
        mint: ctx.accounts.mint.to_account_info().clone(),
        update_authority: ctx.accounts.receipt.to_account_info().clone(),
        mint_authority: ctx.accounts.receipt.to_account_info().clone(),
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

  let verify_signer_seeds: &[&[&[u8]]] = &[token_voter_seeds!(ctx.accounts.token_voter)];

  verify_sized_collection_item(
    CpiContext::new_with_signer(
      ctx
        .accounts
        .token_metadata_program
        .to_account_info()
        .clone(),
      VerifySizedCollectionItem {
        payer: ctx.accounts.payer.to_account_info().clone(),
        metadata: ctx.accounts.metadata.to_account_info().clone(),
        collection_authority: ctx.accounts.token_voter.to_account_info().clone(),
        collection_mint: ctx.accounts.collection.to_account_info().clone(),
        collection_metadata: ctx.accounts.collection_metadata.to_account_info().clone(),
        collection_master_edition: ctx
          .accounts
          .collection_master_edition
          .to_account_info()
          .clone(),
      },
      verify_signer_seeds,
    ),
    None,
  )?;

  Ok(())
}
