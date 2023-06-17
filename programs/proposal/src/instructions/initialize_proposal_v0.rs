use crate::state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeProposalArgsV0 {
  /// Allow a custom seed for indexing
  pub seed: Vec<u8>,
  pub name: String,
  pub uri: String,
  pub choices: Vec<Choice>,
  // Tags which can be used to filter proposals
  pub tags: Vec<String>,
}

#[derive(Accounts)]
#[instruction(args: InitializeProposalArgsV0)]
pub struct InitializeProposalV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  /// Every proposal must have an owner to prevent seed collision
  pub owner: Signer<'info>,
  #[account(
    init,
    payer = payer,
    seeds = [b"proposal", owner.key().as_ref(), &args.seed[..]],
    space = 8 + 60 + args.seed.len() + ProposalV0::INIT_SPACE + args.choices.len() * Choice::INIT_SPACE,
    bump
  )]
  pub proposal: Box<Account<'info, ProposalV0>>,
  pub proposal_config: Box<Account<'info, ProposalConfigV0>>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeProposalV0>, args: InitializeProposalArgsV0) -> Result<()> {
  ctx.accounts.proposal.set_inner(ProposalV0 {
    owner: ctx.accounts.owner.key(),
    state: ProposalState::Voting,
    tags: args.tags,
    created_at: Clock::get()?.unix_timestamp,
    proposal_config: ctx.accounts.proposal_config.key(),
    seed: args.seed,
    name: args.name,
    uri: args.uri,
    choices: args.choices,
    bump_seed: ctx.bumps["proposal"],
  });
  Ok(())
}
