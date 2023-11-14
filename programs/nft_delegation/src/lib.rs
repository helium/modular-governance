use anchor_lang::prelude::*;

declare_id!("nde169MVKXLYbtDbrTF8wiJc8KYJRvLXKAzHApZ5vvD");

pub mod instructions;
pub mod state;

pub use instructions::*;

#[program]
pub mod nft_delegation {
  use super::*;

  pub fn delegate_v0(ctx: Context<DelegateV0>) -> Result<()> {
    delegate_v0::handler(ctx)
  }

  pub fn undelegate_v0(ctx: Context<UndelegateV0>) -> Result<()> {
    undelegate_v0::handler(ctx)
  }
}
