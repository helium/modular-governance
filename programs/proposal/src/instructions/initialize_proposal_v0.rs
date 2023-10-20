use crate::state::*;
use anchor_lang::prelude::*;

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ChoiceArg {
  #[max_len(200)]
  pub name: String,
  /// Any other data that you may want to put in here
  #[max_len(200)]
  pub uri: Option<String>,
}

impl From<ChoiceArg> for Choice {
  fn from(value: ChoiceArg) -> Self {
    Choice {
      weight: 0,
      name: value.name,
      uri: value.uri,
    }
  }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeProposalArgsV0 {
  /// Allow a custom seed for indexing
  pub seed: Vec<u8>,
  pub name: String,
  pub uri: String,
  /// Allows for multiple selection votes
  pub max_choices_per_voter: u16,
  pub choices: Vec<ChoiceArg>,
  // Tags which can be used to filter proposals
  pub tags: Vec<String>,
}

#[derive(Accounts)]
#[instruction(args: InitializeProposalArgsV0)]
pub struct InitializeProposalV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  /// Every proposal must have a namespace to prevent seed collision
  pub namespace: Signer<'info>,
  #[account(
    init,
    payer = payer,
    seeds = [b"proposal", namespace.key().as_ref(), &args.seed[..]],
    space = 8 + 60 + args.seed.len() + ProposalV0::INIT_SPACE + args.choices.len() * Choice::INIT_SPACE + args.tags.iter().map(|tag| tag.len() as usize).sum::<usize>(),
    bump
  )]
  pub proposal: Box<Account<'info, ProposalV0>>,
  /// CHECK: Setting this account, does not need a check. Putting here instead of args to save tx space
  pub owner: UncheckedAccount<'info>,
  pub proposal_config: Box<Account<'info, ProposalConfigV0>>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeProposalV0>, args: InitializeProposalArgsV0) -> Result<()> {
  ctx.accounts.proposal.set_inner(ProposalV0 {
    namespace: ctx.accounts.namespace.key(),
    owner: ctx.accounts.owner.key(),
    state: ProposalState::Draft,
    tags: args.tags,
    max_choices_per_voter: args.max_choices_per_voter,
    created_at: Clock::get()?.unix_timestamp,
    proposal_config: ctx.accounts.proposal_config.key(),
    seed: args.seed,
    name: args.name,
    uri: args.uri,
    choices: args.choices.into_iter().map(|c| c.into()).collect(),
    bump_seed: ctx.bumps["proposal"],
  });
  Ok(())
}
