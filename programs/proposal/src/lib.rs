use anchor_lang::prelude::*;

declare_id!("propXxHSbYTCSwqJA2Vv3Sw27LTJbhBQLSGmmUVZghq");

pub mod errors;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

#[program]
pub mod modular_governance {
    use super::*;

    pub fn initialize_proposal_v0(
        ctx: Context<InitializeProposalV0>,
        args: InitializeProposalArgsV0,
    ) -> Result<()> {
        initialize_proposal_v0::handler(ctx, args)
    }

    pub fn vote_v0(ctx: Context<VoteV0>, args: VoteArgsV0) -> Result<()> {
        vote_v0::handler(ctx, args)
    }

    pub fn resolve_v0(ctx: Context<ResolveV0>, args: ResolveArgsV0) -> Result<()> {
        resolve_v0::handler(ctx, args)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
