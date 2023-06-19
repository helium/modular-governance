use crate::{state::*, wallet_seeds};
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::{prelude::*, solana_program};
use proposal::{ProposalState, ProposalV0};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ExecuteTransactionArgsV0 {
  pub choice: u16,
  pub transaction: u16,
}

#[derive(Accounts)]
#[instruction(args: ExecuteTransactionArgsV0)]
pub struct ExecuteTransactionV0<'info> {
  #[account(
    has_one = wallet
  )]
  pub organization_wallet: Box<Account<'info, OrganizationWalletV0>>,
  #[account(
    constraint = match &proposal.state {
      ProposalState::Resolved(choices) => choices.iter().any(|c| *c == args.choice),
      _ => false,
    },
  )]
  pub proposal: Box<Account<'info, ProposalV0>>,
  #[account(
    has_one = proposal,
    has_one = organization_wallet,
  )]
  pub wallet_proposal: Box<Account<'info, WalletProposalV0>>,
  /// CHECK: Checked via has one
  #[account(mut)]
  pub wallet: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<ExecuteTransactionV0>, args: ExecuteTransactionArgsV0) -> Result<()> {
  let transaction = &ctx.accounts.wallet_proposal.choice_transactions[args.choice as usize]
    [args.transaction as usize];
  let wallet_seeds: &[&[u8]] = wallet_seeds!(ctx.accounts.organization_wallet);

  let prefix: Vec<&[u8]> = vec![
    b"custom",
    ctx
      .accounts
      .organization_wallet
      .to_account_info()
      .key
      .as_ref(),
  ];
  // Need to convert to &[&[u8]] because invoke_signed expects that
  let signers_inner_u8: Vec<Vec<&[u8]>> = transaction
    .signer_seeds
    .iter()
    .map(|s| {
      let mut clone = prefix.clone();
      clone.extend(s.iter().map(|v| v.as_slice()).collect::<Vec<&[u8]>>());

      clone
    })
    .collect();
  let mut signers = signers_inner_u8
    .iter()
    .map(|s| s.as_slice())
    .collect::<Vec<&[&[u8]]>>();

  signers.extend(vec![wallet_seeds]);

  let signer_addresses = signers
    .iter()
    .map(|s| Pubkey::create_program_address(s, ctx.program_id).unwrap())
    .collect::<std::collections::HashSet<Pubkey>>();
  for ix in &transaction.instructions {
    let mut accounts = Vec::new();
    let mut account_infos = Vec::new();
    for i in &ix.accounts {
      let acct = ctx.remaining_accounts[*i as usize].clone();
      accounts.push(acct.clone());
      account_infos.push(AccountMeta {
        pubkey: acct.key(),
        is_signer: acct.key() == ctx.accounts.wallet.key()
          || acct.is_signer
          || signer_addresses.contains(&acct.key()),
        is_writable: acct.is_writable,
      })
    }
    solana_program::program::invoke_signed(
      &Instruction {
        program_id: *ctx.remaining_accounts[ix.program_id_index as usize].key,
        accounts: account_infos,
        data: ix.data.clone(),
      },
      accounts.as_slice(),
      &signers,
    )?;
  }

  Ok(())
}
