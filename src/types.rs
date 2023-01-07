use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Balance, Gas, log};
use near_sdk::collections::{LookupMap, UnorderedMap};
use std::collections::HashMap;

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

/*#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Ord, Eq, PartialEq, PartialOrd, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UniqId(String);*/
pub type UniqId = String;

pub type SalePriceInYoctoNear = U128;
pub type TokenId = String;                          // Same as in Minting Contract
pub type TreeIndex = u64;                           // Unique identifier of an NFT, a number

/// RevenueTable, that is a LookupMap that takes care of limits
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RevenueTable(HashMap<AccountId, u64>);

/// InProgressMetadata
/// Some fields can be null, because it is possible, that the Artist does not finish uploading all the info
/// When it is ready, it will be possible to mint based on the prepairer-data that is here.
/// The data that was already made into an NFT, should be deleted.
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
    /// It is not sure that we will have this field
    pub scheduled: Option<u64>,
    /// Title of the NFT, follows NFT standard
    pub title: Option<String>,
    /// Description of the NFT, follows NFT standard
    pub desc: Option<String>,
    /// Metadata, does not follow standard, here inside this we will have fields that record labels usually use. This is an IPFS CID
    pub meta: Option<String>,
    /// Image, according to NFT standard. It will be an IPFS CID
    pub image: Option<String>,
    /// Music folder, not part of NFT standard. This is an IPFS CID, and it points to a folder that has different quality files of the same song.
    pub music: Option<String>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MintingContractArgs {
    pub receiver_id: AccountId,
    pub metadata: MintingContractMeta,

}

/// Exact copy of TokenMetadata, from Fono-Root (that is following the standard)
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MintingContractMeta {
    pub title: String,                      // This is `title` in InProgressMetadata
    pub description: String,                // This is `desc` in InProgressMetadata
    pub media: String,                      // This is `image` in InProgressMetadata
    pub media_hash: Option<Base64VecU8>,    // We will have to do this later
    pub copies: Option<u64>,                // We will pass None
    pub issued_at: Option<u64>,             // We will pass None
    pub expires_at: Option<u64>,            // We will pass None
    pub starts_at: Option<u64>,             // We will pass None
    pub updated_at: Option<u64>,            // We will pass None
    pub extra: Option<String>,              // This is JSON-stringified version of MintingContractExtra
    pub reference: String,                  // This is `meta` in InProgressMetadata
    pub reference_hash: Option<Base64VecU8>,// We will have to do this later
}

/// Exact copy of Extra, from Fono-Root
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MintingContractExtra {
    pub music_cid: String,                                      // This is `music` in InProgressMetadata
    pub music_hash: Option<Base64VecU8>,                        // We will pass None
    pub parent: Option<TokenId>,                                // We will pass None
    pub instance_nonce: u32,                                    // We will pass a very lare namber, the MintingContract will overwrite it
    pub generation: u32,                                        // We will pass a very lare namber, the MintingContract will overwrite it
}

/// Data that is sent from the front-end, can be half-ready
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftDataFromFrontEnd {
    // **TODO** We will need hash as well.
    pub contract: AccountId,
    pub title: Option<String>,
    pub desc: Option<String>,
    pub image_cid: Option<String>,
    pub music_folder_cid: Option<String>,
    pub meta_json_cid: Option<String>
}


/// Key is the same as TreeIndex
pub type Catalogue = UnorderedMap<TreeIndex, Option<CatalogueEntry>>;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CatalogueEntry {
    pub revenue_table: RevenueTable,     // This is the revenue table that used to be part of the NFT in the FonoRoot contract
    pub price: SalePriceInYoctoNear,
}

// We could easily get all the songs for a given Artist, BUT, how do we get all the songs that exist, in chronological order?
// **TODO** If we move `income` from Catalogue, we need another object for that. Should be very similar to Catalogues/Catalogue
//pub type IncomeTables = HashMap<UniqId, IncomeTable>; TreeMap!!

// **TODO** This is just a placeholder
pub type WeDontKnow = String;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct IncomeTable {
    pub total_income: Balance,
    pub current_balance: Balance,
    // Entries that will tell that this is X song in Y contract
    pub root_id: TokenId,
    pub contract: AccountId,
    pub owner: AccountId,
    //is needed, because we need to know where to find it in the Catalogue. Problem with this: We can't move the Song to another Catalogue, or we have to be very carefull.
    // Timestamp We don't need it, just important that the order does not change. Chronological order
    // The way it will be payed out is dependent on the CatalogueTable, or on other table, so this is not dependent on User. This is independent, each song has it's own table.
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
/*
impl UniqId {
    pub fn new(contract: AccountId, root_id: TokenId) -> Option<Self> {
        let uniq_id: UniqId = format!("{}-{}", contract, root_id);
        Some(uniq_id)
    }
}*/

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
