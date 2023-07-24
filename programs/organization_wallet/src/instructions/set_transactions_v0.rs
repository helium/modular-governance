use crate::error::ErrorCode;
use crate::{resize_to_fit::resize_to_fit, state::*};
use anchor_lang::prelude::*;
use proposal::{ProposalState, ProposalV0};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CompiledTransactionArgV0 {
  // Accounts are ordered as follows:
  // 1. Writable signer accounts
  // 2. Read only signer accounts
  // 3. writable accounts
  // 4. read only accounts
  pub num_rw_signers: u8,
  pub num_ro_signers: u8,
  pub num_rw: u8,
  /// Accounts will come from remaining accounts, which allows for lookup tables
  /// and such to reduce size of txn call here
  pub instructions: Vec<CompiledInstructionV0>,
  pub signer_seeds: Vec<Vec<Vec<u8>>>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct SetTransactionsArgsV0 {
  pub choice_index: u16,
  pub transaction_index: u16,
  // Cannot be executed until this offset from proposal end
  pub allow_execution_offset: u32,
  // Cannot be executed after this offset
  pub disable_execution_offset: u32,
  pub transaction: CompiledTransactionArgV0,
}

#[derive(Accounts)]
#[instruction(args: SetTransactionsArgsV0)]
pub struct SetTransactionsV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  pub owner: Signer<'info>,
  #[account(
    constraint = organization_wallet.proposal_configs.iter().any(|c| *c == proposal.proposal_config) @ ErrorCode::InvalidProposalConfig,
  )]
  pub organization_wallet: Box<Account<'info, OrganizationWalletV0>>,
  #[account(
    has_one = owner,
    constraint = proposal.state == ProposalState::Draft @ ErrorCode::InvalidProposalState,
    constraint = proposal.namespace == organization_wallet.organization @ ErrorCode::InvalidOrganization,
  )]
  pub proposal: Box<Account<'info, ProposalV0>>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 8 + 60 + std::mem::size_of::<WalletProposalV0>() + proposal.choices.len() * 4,
    seeds = [b"wallet_proposal", organization_wallet.key().as_ref(), proposal.key().as_ref()],
    bump,
  )]
  pub wallet_proposal: Box<Account<'info, WalletProposalV0>>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 8 + 60 + std::mem::size_of::<ChoiceTransactionV0>(),
    seeds = [b"choice_transaction", wallet_proposal.key().as_ref(), &args.choice_index.to_le_bytes(), &args.transaction_index.to_le_bytes()],
    bump,
  )]
  pub choice_transaction: Box<Account<'info, ChoiceTransactionV0>>,
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SetTransactionsV0>, args: SetTransactionsArgsV0) -> Result<()> {
  require!(
    ctx
      .accounts
      .wallet_proposal
      .num_transactions_by_choice
      .get(args.choice_index as usize)
      .unwrap_or(&0)
      <= &args.transaction_index,
    ErrorCode::InvalidTransactionIndex
  );
  ctx
    .accounts
    .choice_transaction
    .set_inner(ChoiceTransactionV0 {
      choice_index: args.choice_index,
      organization_wallet: ctx.accounts.organization_wallet.key(),
      proposal: ctx.accounts.proposal.key(),
      wallet_proposal: ctx.accounts.wallet_proposal.key(),
      bump_seed: ctx.bumps["choice_transaction"],
      disable_execution_offset: args.disable_execution_offset,
      allow_execution_offset: args.allow_execution_offset,
      transaction: CompiledTransactionV0 {
        num_rw_signers: args.transaction.num_rw_signers,
        num_ro_signers: args.transaction.num_ro_signers,
        num_rw: args.transaction.num_rw,
        instructions: args.transaction.instructions,
        signer_seeds: args.transaction.signer_seeds,
        accounts: ctx.remaining_accounts.iter().map(|a| a.key()).collect(),
      },
    });
  ctx.accounts.wallet_proposal.set_inner(WalletProposalV0 {
    proposal: ctx.accounts.proposal.key(),
    organization_wallet: ctx.accounts.organization_wallet.key(),
    num_transactions_by_choice: ctx
      .accounts
      .wallet_proposal
      .num_transactions_by_choice
      .iter()
      .enumerate()
      .map(|(i, n)| {
        if i == args.choice_index as usize {
          if args.transaction_index == *n {
            n + 1
          } else {
            *n
          }
        } else {
          *n
        }
      })
      .collect(),
  });

  resize_to_fit(
    &ctx.accounts.payer.to_account_info(),
    &ctx.accounts.system_program.to_account_info(),
    &ctx.accounts.choice_transaction,
  )?;

  Ok(())
}
