use crate::{resize_to_fit::resize_to_fit, state::*};
use anchor_lang::prelude::*;
use organization::state::OrganizationV0;
use proposal::{ProposalState, ProposalV0};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct SetTransactionsArgsV0 {
  pub choice_index: usize,
  pub transaction_index: usize,
  /// Accounts will come from remaining accounts, which allows for lookup tables
  /// and such to reduce size of txn call here
  pub instructions: Vec<CompiledInstruction>,
  pub signer_seeds: Vec<Vec<Vec<u8>>>,
}

#[derive(Accounts)]
#[instruction(args: SetTransactionsArgsV0)]
pub struct SetTransactionsV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  pub authority: Signer<'info>,
  #[account(has_one = authority)]
  pub owner: Account<'info, OrganizationV0>,
  #[account(
    constraint = proposal.state == ProposalState::Draft,
    has_one = owner
  )]
  pub proposal: Box<Account<'info, ProposalV0>>,
  #[account(mut)]
  pub wallet_proposal: Box<Account<'info, WalletProposalV0>>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SetTransactionsV0>, args: SetTransactionsArgsV0) -> Result<()> {
  let mut new_choice_transactions = vec![];
  for i in 0..=args.choice_index {
    new_choice_transactions.push(
      ctx
        .accounts
        .wallet_proposal
        .choice_transactions
        .get(i)
        .map(|i| i.clone())
        .unwrap_or_else(|| vec![]),
    );
  }
  let existing_transactions = ctx
    .accounts
    .wallet_proposal
    .choice_transactions
    .get(args.choice_index)
    .map(|i| i.clone())
    .unwrap_or_else(|| vec![]);
  let mut new_transactions = vec![];
  for j in 0..args.transaction_index {
    new_transactions.push(
      existing_transactions
        .get(j)
        .map(|i| i.clone())
        .unwrap_or_else(|| CompiledTransaction::default()),
    );
  }
  new_transactions.push(CompiledTransaction {
    instructions: args.instructions,
    signer_seeds: args.signer_seeds,
    accounts: ctx.remaining_accounts.iter().map(|a| a.key()).collect(),
  });
  for j in (args.transaction_index + 1)..existing_transactions.len() {
    new_transactions.push(
      existing_transactions
        .get(j)
        .map(|i| i.clone())
        .unwrap_or_else(|| CompiledTransaction::default()),
    );
  }
  new_choice_transactions.push(new_transactions);
  for i in (args.choice_index + 1)..ctx.accounts.wallet_proposal.choice_transactions.len() {
    new_choice_transactions.push(
      ctx
        .accounts
        .wallet_proposal
        .choice_transactions
        .get(i)
        .map(|i| i.clone())
        .unwrap_or_else(|| vec![]),
    );
  }

  ctx.accounts.wallet_proposal.choice_transactions = new_choice_transactions;

  resize_to_fit(
    &ctx.accounts.payer.to_account_info(),
    &ctx.accounts.system_program.to_account_info(),
    &ctx.accounts.wallet_proposal,
  )?;

  Ok(())
}
