use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod resize_to_fit;
pub mod state;

use instructions::*;

declare_id!("org9nsbSiTCJzeApoS2B3uwjM2gbQH48QbUUrhAAjzG");

#[program]
pub mod organization_wallet {
  use super::*;

  pub fn initialize_organization_wallet_v0(
    ctx: Context<InitializeOrganizationWalletV0>,
    args: InitializeOrganizationWalletArgsV0,
  ) -> Result<()> {
    initialize_organization_wallet_v0::handler(ctx, args)
  }

  pub fn initialize_wallet_proposal_v0(ctx: Context<InitializeWalletProposalV0>) -> Result<()> {
    initialize_wallet_proposal_v0::handler(ctx)
  }

  pub fn set_transactions_v0(
    ctx: Context<SetTransactionsV0>,
    args: SetTransactionsArgsV0,
  ) -> Result<()> {
    set_transactions_v0::handler(ctx, args)
  }
}
