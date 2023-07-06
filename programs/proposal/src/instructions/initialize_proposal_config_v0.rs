use crate::state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeProposalConfigArgsV0 {
  pub name: String,
  /// Signer that controls voting and vote weights
  pub vote_controller: Pubkey,
  /// Signer that controls the transitions of `ProposalState`
  /// You can either use the default `state-controller` smart contract
  /// Or you can implement a program that calls the `resolve_v0` method.
  /// The vote can only be resolved when this `resolution_settings` PDA signs `resolve_v0`. This allows
  /// you to trigger resolution on either (a) a vote, (b) a timestamp, or (c) some custom trigger with clockwork
  pub state_controller: Pubkey,
  /// Optional program that will be called with `on_vote_v0` after every vote
  /// Defaults to the owner of `resolution_settings`, which allows it to reactively call resolve_v0
  pub on_vote_hook: Pubkey,
}

#[derive(Accounts)]
#[instruction(args: InitializeProposalConfigArgsV0)]
pub struct InitializeProposalConfigV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  /// Every proposal config must have an owner to prevent seed collision
  pub owner: Signer<'info>,
  #[account(
    init,
    payer = payer,
    seeds = [b"proposal_config", args.name.as_bytes()],
    space = 8 + 60 + args.name.len() + ProposalConfigV0::INIT_SPACE,
    bump
  )]
  pub proposal_config: Box<Account<'info, ProposalConfigV0>>,
  pub system_program: Program<'info, System>,
}

pub fn handler(
  ctx: Context<InitializeProposalConfigV0>,
  args: InitializeProposalConfigArgsV0,
) -> Result<()> {
  ctx.accounts.proposal_config.set_inner(ProposalConfigV0 {
    name: args.name,
    vote_controller: args.vote_controller,
    state_controller: args.state_controller,
    on_vote_hook: args.on_vote_hook,
    bump_seed: ctx.bumps["proposal_config"],
  });
  Ok(())
}
