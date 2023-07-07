use crate::metaplex::{burn, Burn, Metadata};
use crate::receipt_seeds;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, CloseAccount, Transfer};
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct WithdrawV0<'info> {
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

  // checking the PDA address it just an extra precaution,
  // the other constraints must be exhaustive
  #[account(
    mut,
    has_one = token_voter,
    has_one = mint,
    close = refund,
    seeds = [b"receipt".as_ref(), mint.key().as_ref()],
    bump = receipt.bump_seed,
  )]
  pub receipt: Box<Account<'info, ReceiptV0>>,
  #[account(mut)]
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
    mut,
    associated_token::mint = mint,
    associated_token::authority = owner,
  )]
  pub receipt_token_account: Box<Account<'info, TokenAccount>>,

  #[account(
    mut,
    close = refund,
    associated_token::authority = receipt,
    associated_token::mint = deposit_mint,
  )]
  pub vault: Box<Account<'info, TokenAccount>>,
  #[account(
    init_if_needed,
    associated_token::authority = owner,
    associated_token::mint = deposit_mint,
    payer = payer
  )]
  pub token_account: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub payer: Signer<'info>,
  /// CHECK: Not needed because it just gets sol
  #[account(mut)]
  pub refund: UncheckedAccount<'info>,
  pub owner: Signer<'info>,

  pub deposit_mint: Box<Account<'info, Mint>>,

  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_metadata_program: Program<'info, Metadata>,
  pub rent: Sysvar<'info, Rent>,
}

impl<'info> WithdrawV0<'info> {
  fn burn_nft_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
    let cpi_accounts = Burn {
      metadata: self.metadata.to_account_info(),
      owner: self.owner.to_account_info(),
      mint: self.mint.to_account_info(),
      token_account: self.receipt_token_account.to_account_info(),
      master_edition_account: self.master_edition.to_account_info(),
      collection: self.collection_metadata.to_account_info(),
    };
    CpiContext::new(self.token_metadata_program.to_account_info(), cpi_accounts)
  }

  fn withdraw_transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
    let cpi_accounts = Transfer {
      from: self.vault.to_account_info(),
      to: self.token_account.to_account_info(),
      authority: self.receipt.to_account_info(),
    };
    CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
  }

  fn withdraw_close_ctx(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
    let cpi_accounts = CloseAccount {
      authority: self.receipt.to_account_info(),
      account: self.vault.to_account_info(),
      destination: self.refund.to_account_info(),
    };
    CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
  }
}

pub fn handler(ctx: Context<WithdrawV0>) -> Result<()> {
  let signer_seeds: &[&[&[u8]]] = &[receipt_seeds!(ctx.accounts.receipt)];

  burn(ctx.accounts.burn_nft_ctx())?;
  token::transfer(
    ctx
      .accounts
      .withdraw_transfer_ctx()
      .with_signer(signer_seeds),
    ctx.accounts.receipt.amount,
  )?;
  token::close_account(ctx.accounts.withdraw_close_ctx().with_signer(signer_seeds))?;

  Ok(())
}
