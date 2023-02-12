use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Balance, Gas, log};
use near_sdk::collections::{UnorderedMap};
use std::collections::HashMap;
use std::ops::Deref;
use regex::Regex;

/// Account ID used for $NEAR in near-sdk v3.
/// Need to keep it around for backward compatibility.
pub const OLD_BASE_TOKEN: &str = "";

/// Account ID that represents a token in near-sdk v3.
/// Need to keep it around for backward compatibility.
pub type OldAccountId = String;

/// 1 yN to prevent access key fraud.
pub const ONE_YOCTO_NEAR: Balance = 1;

/// Gas for single ft_transfer call.
pub const GAS_FOR_FT_TRANSFER: Gas = Gas(10_000_000_000_000);

/// Configuration of the DAO.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
    /// Name of the DAO.
    pub name: String,
    /// Purpose of this DAO.
    pub purpose: String,
    /// Generic metadata. Can be used by specific UI to store additional data.
    /// This is not used by anything in the contract.
    pub metadata: Base64VecU8,
}

#[cfg(test)]
impl Config {
    pub fn test_config() -> Self {
        Self {
            name: "Test".to_string(),
            purpose: "to test".to_string(),
            metadata: Base64VecU8(vec![]),
        }
    }
}

/// Set of possible action to take.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Action {
    /// Action to add proposal. Used internally.
    AddProposal,
    /// Action to remove given proposal. Used for immediate deletion in special cases.
    RemoveProposal,
    /// Vote to approve given proposal or bounty.
    VoteApprove,
    /// Vote to reject given proposal or bounty.
    VoteReject,
    /// Vote to remove given proposal or bounty (because it's spam).
    VoteRemove,
    /// Finalize proposal, called when it's expired to return the funds
    /// (or in the future can be used for early proposal closure).
    Finalize,
    /// Move a proposal to the hub to shift into another DAO.
    MoveToHub,
}

/// Combination of MintingContract+RootId (e.g. "minting-contract.near-fono-root-2")
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct UniqId(String);

pub type SalePriceInYoctoNear = U128;

/// This is the same variable that is used in the FonoRoot minting contract (e.g. "fono-root-2-5", "fono-root-2")
pub type TokenId = String;

/// Unique identifier of an NFT, a number. A UniqId will map to a TreeIndex
pub type TreeIndex = u64;

/// RevenueTable, that is a LookupMap that takes care of limits
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RevenueTable(HashMap<AccountId, u64>);

/// InProgressMetadata
/// Some fields can be null, because it is possible, that the Artist does not finish uploading all the info
/// When it is ready, it will be possible to mint based on the prepaired-data that is here.
/// The data that was already made into an NFT will be deleted.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct InProgressMetadata {
    /// ID of the InProgressMetadata
    pub id: u64,
    /// Timestamp, env::block_timestamp()
    pub initiated: u64,
    /// Account ID of the minter, usually the Artist.
    pub artist: AccountId,
    /// Minting contract. The NFT will live on this contract.
    pub contract: AccountId,
    /// **TODO** It is not sure that we will have this field. For this, we would need to implement the ScheduleMint proposal.
    pub scheduled: Option<u64>,
    /// Title of the NFT, follows NFT standard
    pub title: Option<String>,
    /// Description of the NFT, follows NFT standard
    pub desc: Option<String>,
    /// Metadata, does not follow standard, here inside this we will have fields that record labels usually use. This is an IPFS CID
    pub meta: Option<String>,
    /// SHA256 hash of the meta object. Has to be Some, if meta is supplied
    pub meta_hash: Option<Base64VecU8>,
    /// Image, according to NFT standard. It will be an IPFS CID
    pub image: Option<String>,
    /// SHA256 hash of the image. Has to be Some, if image is supplied
    pub image_hash: Option<Base64VecU8>,
    /// Music folder, not part of NFT standard. This is an IPFS CID, and it points to a folder that has different quality files of the same song.
    pub music: Option<String>,          // currently this is a music file, but we want to change that in the future
    /// SHA256 hash of the music folder. Has to be Some, if music is supplied
    pub music_hash: Option<Base64VecU8>,
    /// Single music file that is compatible with OpenSea
    pub animation_url: Option<String>,
    /// SHA256 hash of the music. Has to be Some if music is supplied
    pub animation_url_hash: Option<Base64VecU8>
}

/// These are the parameters the FonoRoot minting contract will take as input, when we mint a new RootNFT
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MintingContractArgs {
    pub receiver_id: AccountId,
    pub metadata: MintingContractMeta,
}

/// These are the parameters the FonoRoot minting contract will take as input, when we buy an NFT
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct BuyArgs {
    pub root_id: TokenId
}

/// Exact copy of TokenMetadata, from FonoRoot (that is following the NFT standard)
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MintingContractMeta {
    pub title: String,                      // This is `title` in InProgressMetadata
    pub description: String,                // This is `desc` in InProgressMetadata
    pub media: String,                      // This is `image` in InProgressMetadata
    pub media_hash: Option<Base64VecU8>,    // This is `image_hash` in InProgressMetadata
    pub copies: Option<u64>,                // We will pass None
    pub issued_at: Option<u64>,             // We will pass None
    pub expires_at: Option<u64>,            // We will pass None
    pub starts_at: Option<u64>,             // We will pass None
    pub updated_at: Option<u64>,            // We will pass None
    pub extra: Option<String>,              // This is JSON-stringified version of MintingContractExtra
    pub reference: String,                  // This is `meta` in InProgressMetadata
    pub reference_hash: Option<Base64VecU8>,// This is `meta_hash` in InProgressMetadata
}

/// Exact copy of Extra, from Fono-Root
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MintingContractExtra {
    pub music_cid: Option<String>,                              // This is `music` in InProgressMetadata
    pub music_hash: Option<Base64VecU8>,                        // This is `music_hash` in InProgressMetadata
    pub animation_url: Option<String>,                          // This is animation_url in InProgressMetadata
    pub animation_url_hash: Option<Base64VecU8>,                // This is animation_url_hash in InProgressMetadata
    pub parent: Option<TokenId>,                                // We will pass None
    pub instance_nonce: u32,                                    // We will pass a very lare namber, the MintingContract will overwrite it
    pub generation: u32,                                        // We will pass a very lare namber, the MintingContract will overwrite it
    pub next_buyable: Option<u32>                               // We will pass None
}

/// Data that is sent from the front end, can be half-ready
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftDataFromFrontEnd {
    pub contract: AccountId,
    pub title: Option<String>,
    pub desc: Option<String>,
    pub image_cid: Option<String>,
    pub image_hash: Option<Base64VecU8>,                        // If image exists, we need hash for it as well
    pub music_folder_cid: Option<String>,
    pub music_folder_hash: Option<Base64VecU8>,                 // If music exists, we need hash for it as well
    pub animation_url: Option<String>,
    pub animation_url_hash: Option<Base64VecU8>,                // If animation_url exists, we need has for it as well
    pub meta_json_cid: Option<String>,
    pub meta_json_hash: Option<Base64VecU8>                     // If meta exists, we need hash for it as well
}

/// A Catalogue for an Artist. Each Artist has a Catalogue. In Contract, catalogues LookupMap is a list if Catalogue-s. (Artist AccountId is key)
/// TreeIndex is unique, we can find the NFT in the income_tables with this index
pub type Catalogue = UnorderedMap<TreeIndex, Option<CatalogueEntry>>;

/// Each song has a CatalogueEntry, where there is a list of accounts that should be paid out from the income (with percentages)
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CatalogueEntry {                                     // Price used to be here, that's why this is a struct
    pub revenue_table: RevenueTable,                            // We will keep this a struct, because we might add more fields later.
}

// **TODO** This is just a placeholder
// **TODO** We either need to keep ScheduleMint, or we need some kind of special Role, like CronCat, which is allowed to mint, even if it is not the Artist.
pub type ScheduleMintParams = String;

/// IncomeTable is a very important object, that is used for other things as well, because this is most easily iterable.
/// This is why the price is here. It might be a good idea to rename this, to something like _MainSongObject_
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct IncomeTable {
    pub total_income: Balance,
    pub current_balance: Balance,
    pub root_id: TokenId,
    pub contract: AccountId,
    pub owner: AccountId,
    pub price: Option<SalePriceInYoctoNear>,
}

/// Payout object
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
} 

/// Failed Transaction object
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct FailedTransaction {
    pub beneficiary: AccountId,
    pub amount: Balance
}

/// Return value of `mint_root`, from Fono-Root minting contract
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MintRootResult {
    pub contract: AccountId,                                        // Name of the minting contract
    pub root_id: TokenId,                                           // ID of the RootNFT
}


impl Action {
    pub fn to_policy_label(&self) -> String {
        format!("{:?}", self)
    }
}

impl RevenueTable {
    pub fn new(map: HashMap<AccountId, u64>) -> Option<Self> {
        let sum: u64 = map.values().sum();
        let max_length = 16;

        // RevenueTable is only valid if total values adds up to 100%, and length of the list is below limit
        if sum == 10000 && map.len() < max_length {
            Some(RevenueTable(map))
        } else {
            log!("Revenue Table is not valid!");
            None
        }
    }
}
impl IntoIterator for RevenueTable {
    type Item = (AccountId, u64);
    type IntoIter = std::collections::hash_map::IntoIter<AccountId, u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl Deref for RevenueTable {
    type Target = HashMap<AccountId, u64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UniqId {
    pub fn new(contract: AccountId, root_id: TokenId) -> UniqId {
        let root_id_regex = Regex::new(r"fono-root-[0-9]{1,}").unwrap();
        assert!(root_id_regex.is_match(&root_id), "RootID is not valid!");
        assert!(contract.to_string().parse::<AccountId>().is_ok(), "AccountId is not valid!");
        UniqId(format!("{}-{}", contract, root_id))
    }
}

/// In near-sdk v3, the token was represented by a String, with no other restrictions.
/// That being said, Sputnik used "" (empty String) as a convention to represent the $NEAR token.
/// In near-sdk v4, the token representation was replaced by AccountId (which is in fact a wrapper
/// over a String), with the restriction that the token must be between 2 and 64 chars.
/// Sputnik had to adapt since "" was not allowed anymore and we chose to represent the token as a
/// Option<AccountId> with the convention that None represents the $NEAR token.
/// This function is required to help with the transition and keep the backward compatibility.
pub fn convert_old_to_new_token(old_account_id: &OldAccountId) -> Option<AccountId> {
    if old_account_id == OLD_BASE_TOKEN {
        return None;
    }
    Some(AccountId::new_unchecked(old_account_id.clone()))
}
