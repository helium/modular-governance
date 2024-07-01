use anchor_lang::prelude::*;

declare_id!("nprx42sXf5rpVnwBWEdRg1d8tuCWsTuVLys1pRWwE6p");

pub mod error;
pub mod instructions;
pub mod resize_to_fit;
pub mod state;

pub use instructions::*;

#[program]
pub mod nft_proxy {
  use super::*;

  pub fn initialize_proxy_config_v0(
    ctx: Context<InitializeProxyConfigV0>,
    args: InitializeProxyConfigArgsV0,
  ) -> Result<()> {
    initialize_proxy_config_v0::handler(ctx, args)
  }

  pub fn assign_proxy_v0(ctx: Context<AssignProxyV0>, args: AssignProxyArgsV0) -> Result<()> {
    assign_proxy_v0::handler(ctx, args)
  }

  pub fn unassign_proxy_v0(ctx: Context<UnassignProxyV0>) -> Result<()> {
    unassign_proxy_v0::handler(ctx)
  }

  pub fn update_proxy_config_v0(
    ctx: Context<UpdateProxyConfigV0>,
    args: UpdateProxyConfigArgsV0,
  ) -> Result<()> {
    update_proxy_config_v0::handler(ctx, args)
  }
}
