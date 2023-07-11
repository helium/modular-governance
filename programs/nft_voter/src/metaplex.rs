use std::{ops::Deref, str::FromStr};

use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Key {
  Uninitialized,
  EditionV1,
  MasterEditionV1,
  ReservationListV1,
  MetadataV1,
  ReservationListV2,
  MasterEditionV2,
  EditionMarker,
  UseAuthorityRecord,
  CollectionAuthorityRecord,
  TokenOwnedEscrow,
  TokenRecord,
  MetadataDelegate,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, PartialEq, Eq)]
pub struct Metadata {
  /// Account discriminator.
  pub key: Key,
  /// Address of the update authority.
  #[cfg_attr(feature = "serde-feature", serde(with = "As::<DisplayFromStr>"))]
  pub update_authority: Pubkey,
  /// Address of the mint.
  #[cfg_attr(feature = "serde-feature", serde(with = "As::<DisplayFromStr>"))]
  pub mint: Pubkey,
  /// Asset data.
  pub data: Data,
  // Immutable, once flipped, all sales of this metadata are considered secondary.
  pub primary_sale_happened: bool,
  // Whether or not the data struct is mutable, default is not
  pub is_mutable: bool,
  /// nonce for easy calculation of editions, if present
  pub edition_nonce: Option<u8>,
  pub token_standard: Option<TokenStandard>,
  /// Collection
  pub collection: Option<Collection>,
  /// Uses
  pub uses: Option<Uses>,
  /// Collection Details
  pub collection_details: Option<CollectionDetails>,
  /// Programmable Config
  pub programmable_config: Option<ProgrammableConfig>,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum CollectionDetails {
  V1 { size: u64 },
}

/// Configuration for programmable assets.
#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum ProgrammableConfig {
  V1 {
    /// Programmable authorization rules.
    #[cfg_attr(
      feature = "serde-feature",
      serde(
        deserialize_with = "deser_option_pubkey",
        serialize_with = "ser_option_pubkey"
      )
    )]
    rule_set: Option<Pubkey>,
  },
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

#[derive(Clone, Debug, PartialEq)]
pub struct MetadataAccount(Metadata);

impl anchor_lang::AccountDeserialize for MetadataAccount {
  fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
    let md = Self::try_deserialize_unchecked(buf)?;
    Ok(md)
  }

  fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
    let md = Metadata::deserialize(buf)?;
    Ok(Self(md))
  }
}

impl anchor_lang::AccountSerialize for MetadataAccount {}

impl anchor_lang::Owner for MetadataAccount {
  fn owner() -> Pubkey {
    Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap()
  }
}

impl Deref for MetadataAccount {
  type Target = Metadata;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
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
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokenStandard {
  NonFungible,             // This is a master edition
  FungibleAsset,           // A token with metadata that can also have attributes
  Fungible,                // A token with simple metadata
  NonFungibleEdition,      // This is a limited edition
  ProgrammableNonFungible, // NonFungible with programmable configuration
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
#[derive(BorshSerialize, BorshDeserialize, Default, PartialEq, Eq, Debug, Clone)]
pub struct Data {
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
}
