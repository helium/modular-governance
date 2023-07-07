use std::str::FromStr;

use anchor_lang::{
  prelude::*,
  solana_program::{self, instruction::Instruction},
};
use anchor_spl::token::spl_token;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Accounts)]
pub struct Burn<'info> {
  /// CHECK: No
  pub metadata: AccountInfo<'info>,
  /// CHECK: No
  pub owner: AccountInfo<'info>,
  /// CHECK: No
  pub mint: AccountInfo<'info>,
  /// CHECK: No
  pub token_account: AccountInfo<'info>,
  /// CHECK: No
  pub master_edition_account: AccountInfo<'info>,
  /// CHECK: No
  pub collection: AccountInfo<'info>,
}

pub fn burn<'info>(ctx: CpiContext<'_, '_, '_, 'info, Burn<'info>>) -> Result<()> {
  let ix = burn_impl(
    Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap(),
    *ctx.accounts.metadata.key,
    *ctx.accounts.owner.key,
    *ctx.accounts.mint.key,
    *ctx.accounts.token_account.key,
    *ctx.accounts.master_edition_account.key,
    *ctx.accounts.collection.key,
  );
  solana_program::program::invoke_signed(
    &ix,
    &ToAccountInfos::to_account_infos(&ctx),
    ctx.signer_seeds,
  )
  .map_err(Into::into)
}

#[allow(clippy::too_many_arguments)]
pub fn burn_impl(
  program_id: Pubkey,
  metadata: Pubkey,
  owner: Pubkey,
  mint: Pubkey,
  token_account: Pubkey,
  master_edition_account: Pubkey,
  collection: Pubkey,
) -> Instruction {
  let accounts = vec![
    AccountMeta::new(metadata, false),
    AccountMeta::new(owner, true),
    AccountMeta::new(mint, false),
    AccountMeta::new(token_account, false),
    AccountMeta::new(master_edition_account, false),
    AccountMeta::new_readonly(spl_token::id(), false),
    AccountMeta::new(collection, false),
  ];

  Instruction {
    program_id,
    accounts,
    data: MetadataInstruction::BurnNft.try_to_vec().unwrap(),
  }
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct DataV2 {
  /// The name of the asset
  pub name: String,
  /// The symbol for the asset
  pub symbol: String,
  /// URI pointing to JSON representing the asset
  pub uri: String,
  /// Royalty basis points that goes to creators in secondary sales (0-10000)
  pub seller_fee_basis_points: u16,
  /// Array of creators, optional
  pub creators: Option<Vec<Creator>>,
  /// Collection
  pub collection: Option<Collection>,
  /// Uses
  pub uses: Option<Uses>,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct Uses {
  // 17 bytes + Option byte
  pub use_method: UseMethod, //1
  pub remaining: u64,        //8
  pub total: u64,            //8
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum UseMethod {
  Burn,
  Multiple,
  Single,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub struct Creator {
  #[cfg_attr(feature = "serde-feature", serde(with = "As::<DisplayFromStr>"))]
  pub address: Pubkey,
  pub verified: bool,
  // In percentages, NOT basis points ;) Watch out!
  pub share: u8,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct Collection {
  pub verified: bool,
  #[cfg_attr(feature = "serde-feature", serde(with = "As::<DisplayFromStr>"))]
  pub key: Pubkey,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum CollectionDetails {
  V1 { size: u64 },
}

#[derive(Accounts)]
pub struct CreateMetadataAccountsV3<'info> {
  /// CHECK: No
  pub metadata: AccountInfo<'info>,
  /// CHECK: No
  pub mint: AccountInfo<'info>,
  /// CHECK: No
  pub mint_authority: AccountInfo<'info>,
  /// CHECK: No
  pub payer: AccountInfo<'info>,
  /// CHECK: No
  pub update_authority: AccountInfo<'info>,
  /// CHECK: No
  pub system_program: AccountInfo<'info>,
  /// CHECK: No
  pub rent: AccountInfo<'info>,
}

pub fn create_metadata_accounts_v3<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, CreateMetadataAccountsV3<'info>>,
  data: DataV2,
  is_mutable: bool,
  update_authority_is_signer: bool,
  details: Option<CollectionDetails>,
) -> Result<()> {
  let DataV2 {
    name,
    symbol,
    uri,
    creators,
    seller_fee_basis_points,
    collection,
    uses,
  } = data;
  let ix = create_metadata_accounts_v3_impl(
    Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap(),
    *ctx.accounts.metadata.key,
    *ctx.accounts.mint.key,
    *ctx.accounts.mint_authority.key,
    *ctx.accounts.payer.key,
    *ctx.accounts.update_authority.key,
    name,
    symbol,
    uri,
    creators,
    seller_fee_basis_points,
    update_authority_is_signer,
    is_mutable,
    collection,
    uses,
    details,
  );
  solana_program::program::invoke_signed(
    &ix,
    &ToAccountInfos::to_account_infos(&ctx),
    ctx.signer_seeds,
  )
  .map_err(Into::into)
}

#[derive(Clone)]
pub struct Metadata;

impl anchor_lang::Id for Metadata {
  fn id() -> Pubkey {
    Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap()
  }
}

#[derive(Accounts)]
pub struct CreateMasterEditionV3<'info> {
  /// CHECK: No
  pub edition: AccountInfo<'info>,
  /// CHECK: No
  pub mint: AccountInfo<'info>,
  /// CHECK: No
  pub update_authority: AccountInfo<'info>,
  /// CHECK: No
  pub mint_authority: AccountInfo<'info>,
  /// CHECK: No
  pub payer: AccountInfo<'info>,
  /// CHECK: No
  pub metadata: AccountInfo<'info>,
  /// CHECK: No
  pub token_program: AccountInfo<'info>,
  /// CHECK: No
  pub system_program: AccountInfo<'info>,
  /// CHECK: No
  pub rent: AccountInfo<'info>,
}

pub fn create_master_edition_v3<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, CreateMasterEditionV3<'info>>,
  max_supply: Option<u64>,
) -> Result<()> {
  let ix = create_master_edition_v3_impl(
    Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap(),
    *ctx.accounts.edition.key,
    *ctx.accounts.mint.key,
    *ctx.accounts.update_authority.key,
    *ctx.accounts.mint_authority.key,
    *ctx.accounts.metadata.key,
    *ctx.accounts.payer.key,
    max_supply,
  );
  solana_program::program::invoke_signed(
    &ix,
    &ToAccountInfos::to_account_infos(&ctx),
    ctx.signer_seeds,
  )
  .map_err(Into::into)
}

pub fn verify_sized_collection_item<'info>(
  ctx: CpiContext<'_, '_, '_, 'info, VerifySizedCollectionItem<'info>>,
  collection_authority_record: Option<Pubkey>,
) -> Result<()> {
  let ix = verify_sized_collection_item_impl(
    Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap(),
    *ctx.accounts.metadata.key,
    *ctx.accounts.collection_authority.key,
    *ctx.accounts.payer.key,
    *ctx.accounts.collection_mint.key,
    *ctx.accounts.collection_metadata.key,
    *ctx.accounts.collection_master_edition.key,
    collection_authority_record,
  );
  solana_program::program::invoke_signed(
    &ix,
    &ToAccountInfos::to_account_infos(&ctx),
    ctx.signer_seeds,
  )
  .map_err(Into::into)
}

#[derive(Accounts)]
pub struct VerifySizedCollectionItem<'info> {
  /// CHECK: No
  pub payer: AccountInfo<'info>,
  /// CHECK: No
  pub metadata: AccountInfo<'info>,
  /// CHECK: No
  pub collection_authority: AccountInfo<'info>,
  /// CHECK: No
  pub collection_mint: AccountInfo<'info>,
  /// CHECK: No
  #[account(mut)]
  pub collection_metadata: AccountInfo<'info>,
  /// CHECK: No
  pub collection_master_edition: AccountInfo<'info>,
}

//# Create Metadata Accounts V3 -- Supports v1.3 Collection Details
///
///Create a new Metadata Account
///
/// ### Accounts:
///
///   0. `[writable]` Metadata account
///   1. `[]` Mint account
///   2. `[signer]` Mint authority
///   3. `[signer]` payer
///   4. `[signer]` Update authority
///   5. `[]` System program
///   6. Optional `[]` Rent sysvar
///
/// Creates an CreateMetadataAccounts instruction
#[allow(clippy::too_many_arguments)]
pub fn create_metadata_accounts_v3_impl(
  program_id: Pubkey,
  metadata_account: Pubkey,
  mint: Pubkey,
  mint_authority: Pubkey,
  payer: Pubkey,
  update_authority: Pubkey,
  name: String,
  symbol: String,
  uri: String,
  creators: Option<Vec<Creator>>,
  seller_fee_basis_points: u16,
  update_authority_is_signer: bool,
  is_mutable: bool,
  collection: Option<Collection>,
  uses: Option<Uses>,
  collection_details: Option<CollectionDetails>,
) -> Instruction {
  Instruction {
    program_id,
    accounts: vec![
      AccountMeta::new(metadata_account, false),
      AccountMeta::new_readonly(mint, false),
      AccountMeta::new_readonly(mint_authority, true),
      AccountMeta::new(payer, true),
      AccountMeta::new_readonly(update_authority, update_authority_is_signer),
      AccountMeta::new_readonly(solana_program::system_program::id(), false),
    ],
    data: MetadataInstruction::CreateMetadataAccountV3(CreateMetadataAccountArgsV3 {
      data: DataV2 {
        name,
        symbol,
        uri,
        seller_fee_basis_points,
        creators,
        collection,
        uses,
      },
      is_mutable,
      collection_details,
    })
    .try_to_vec()
    .unwrap(),
  }
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct CreateMasterEditionArgs {
  /// If set, means that no more than this number of editions can ever be minted. This is immutable.
  pub max_supply: Option<u64>,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
/// Args for create call
pub struct CreateMetadataAccountArgsV3 {
  /// Note that unique metadatas are disabled for now.
  pub data: DataV2,
  /// Whether you want your metadata to be updateable in the future.
  pub is_mutable: bool,
  /// If this is a collection parent NFT.
  pub collection_details: Option<CollectionDetails>,
}

/// # Verify Collection V2 -- Supports v1.3 Collection Details
///
/// If a MetadataAccount Has a Collection allow the UpdateAuthority of the Collection to Verify the NFT Belongs in the Collection
///
/// ### Accounts:
///
///   0. `[writable]` Metadata account
///   1. `[signer]` Collection Update authority
///   2. `[signer]` payer
///   3. `[]` Mint of the Collection
///   4. `[writable]` Metadata Account of the Collection
///   5. `[]` MasterEdition2 Account of the Collection Token
#[allow(clippy::too_many_arguments)]
pub fn verify_sized_collection_item_impl(
  program_id: Pubkey,
  metadata: Pubkey,
  collection_authority: Pubkey,
  payer: Pubkey,
  collection_mint: Pubkey,
  collection: Pubkey,
  collection_master_edition_account: Pubkey,
  collection_authority_record: Option<Pubkey>,
) -> Instruction {
  let mut accounts = vec![
    AccountMeta::new(metadata, false),
    AccountMeta::new_readonly(collection_authority, true),
    AccountMeta::new(payer, true),
    AccountMeta::new_readonly(collection_mint, false),
    AccountMeta::new(collection, false),
    AccountMeta::new_readonly(collection_master_edition_account, false),
  ];

  if let Some(record) = collection_authority_record {
    accounts.push(AccountMeta::new_readonly(record, false));
  }

  Instruction {
    program_id,
    accounts,
    data: MetadataInstruction::VerifySizedCollectionItem
      .try_to_vec()
      .unwrap(),
  }
}

/// creates a create_master_edition instruction
#[allow(clippy::too_many_arguments)]
pub fn create_master_edition_v3_impl(
  program_id: Pubkey,
  edition: Pubkey,
  mint: Pubkey,
  update_authority: Pubkey,
  mint_authority: Pubkey,
  metadata: Pubkey,
  payer: Pubkey,
  max_supply: Option<u64>,
) -> Instruction {
  let accounts = vec![
    AccountMeta::new(edition, false),
    AccountMeta::new(mint, false),
    AccountMeta::new_readonly(update_authority, true),
    AccountMeta::new_readonly(mint_authority, true),
    AccountMeta::new(payer, true),
    AccountMeta::new(metadata, false),
    AccountMeta::new_readonly(spl_token::id(), false),
    AccountMeta::new_readonly(solana_program::system_program::id(), false),
  ];

  Instruction {
    program_id,
    accounts,
    data: MetadataInstruction::CreateMasterEditionV3(CreateMasterEditionArgs { max_supply })
      .try_to_vec()
      .unwrap(),
  }
}

#[repr(C)]
/// Instructions supported by the Metadata program.
#[derive(BorshSerialize, BorshDeserialize, Clone)]
#[rustfmt::skip]
enum MetadataInstruction {
    CreateMetadataAccount,
    UpdateMetadataAccount,
    DeprecatedCreateMasterEdition,
    DeprecatedMintNewEditionFromMasterEditionViaPrintingToken,
    UpdatePrimarySaleHappenedViaToken,
    DeprecatedSetReservationList,
    DeprecatedCreateReservationList,
    SignMetadata,
    DeprecatedMintPrintingTokensViaToken,
    DeprecatedMintPrintingTokens,
    CreateMasterEdition,
    MintNewEditionFromMasterEditionViaToken,
    ConvertMasterEditionV1ToV2,
    MintNewEditionFromMasterEditionViaVaultProxy,
    PuffMetadata,
    UpdateMetadataAccountV2,
    CreateMetadataAccountV2,
    CreateMasterEditionV3(CreateMasterEditionArgs),
    VerifyCollection,
    Utilize,
    ApproveUseAuthority,
    RevokeUseAuthority,
    UnverifyCollection,
    ApproveCollectionAuthority,
    RevokeCollectionAuthority,
    SetAndVerifyCollection,
    FreezeDelegatedAccount,
    ThawDelegatedAccount,
    RemoveCreatorVerification,
    BurnNft,
    VerifySizedCollectionItem,
    UnverifySizedCollectionItem,
    SetAndVerifySizedCollectionItem,
    CreateMetadataAccountV3(CreateMetadataAccountArgsV3)
}
