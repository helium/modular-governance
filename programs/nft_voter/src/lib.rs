use anchor_lang::prelude::*;

declare_id!("nftvLQ5t6xe2nQF1NBmBBmn15ed59tU6vSCkwQNEqdc");

pub mod error;
pub mod instructions;
pub mod metaplex;
pub mod state;

pub use instructions::*;

#[program]
pub mod nft_voter {
  use super::*;

  pub fn initialize_nft_voter_v0(
    ctx: Context<InitializeNftVoterV0>,
    args: InitializeNftVoterArgsV0,
  ) -> Result<()> {
    initialize_nft_voter_v0::handler(ctx, args)
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
}
