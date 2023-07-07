use anchor_lang::prelude::*;

declare_id!("tokv9Lz2ZeH2F2qPcLokjoNPuvwNJ9gdZ3DaVQLPJcV");

pub mod error;
pub mod instructions;
pub mod metaplex;
pub mod state;

pub use instructions::*;

#[program]
pub mod token_voter {
  use super::*;

  pub fn initialize_token_voter_v0(
    ctx: Context<InitializeTokenVoterV0>,
    args: InitializeTokenVoterArgsV0,
  ) -> Result<()> {
    initialize_token_voter_v0::handler(ctx, args)
  }

  pub fn deposit_v0(ctx: Context<DepositV0>, args: DepositArgsV0) -> Result<()> {
    deposit_v0::handler(ctx, args)
  }

  pub fn relinquish_vote_v0(
    ctx: Context<RelinquishVoteV0>,
    args: RelinquishVoteArgsV0,
  ) -> Result<()> {
    relinquish_vote_v0::handler(ctx, args)
  }

  pub fn vote_v0(ctx: Context<VoteV0>, args: VoteArgsV0) -> Result<()> {
    vote_v0::handler(ctx, args)
  }

  pub fn withdraw_v0(ctx: Context<WithdrawV0>) -> Result<()> {
    withdraw_v0::handler(ctx)
  }
}
