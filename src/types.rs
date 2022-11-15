use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Balance, Gas};
use near_sdk::collections::{LookupMap};

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

// **TODO** Implement UniqId
pub type UniqId = String;             //  ! Not implemented!!

pub type SalePriceInYoctoNear = U128;
pub type TokenId = String;                          // Same as in Minting Contract

// **TODO** Implement InProgressMetadata here
pub struct InProgressMetadata {
    // **TODO** Add fields like `contract`, `image`, `music_folder`, `metadata_json`, `revenue_table`, etc. And `master`/`artist`. 
    // These fields can be null, because it is possible, that the Artist does not finish uploading these info, and it will be still here, half ready.
    // When it is ready, it will be possible to mint based on the prepairer-data that is here.
    // The data that was already made into an NFT, should be deleted.
}

// **TODO** We should have a big Catalogues obejct, something like this:
//pub type Catalogues = LookupMap<AccountId, Catalogue>; // or this goes to lib.rs
// We could easily get all the songs for a given Artist, BUT, how do we get all the songs that exist, in chronological order?

pub type Catalogue = LookupMap<UniqId, CatalogueEntry>;

pub struct CatalogueEntry {
    pub revenue_table: LookupMap<AccountId, u64>,     // This is the revenue table that used to be part of the NFT in the FonoRoot contract
    pub price: SalePriceInYoctoNear,
}

// **TODO** If we move `income` from Catalogue, we need another object for that. Should be very similar to Catalogues/Catalogue
//pub type IncomeTables = HashMap<UniqId, IncomeTable>; TreeMap!!

// **TODO** This is just a placeholder
pub type WeDontKnow = String;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct IncomeTable {
    pub total_income: Balance,
    pub current_balance: Balance,
    // Entries that will tell that this is X song in Y contract
    pub root_id: TokenId,
    pub contract: AccountId,
    // owner: is needed, because we need to know where to find it in the Catalogue. Problem with this: We can't move the Song to another Catalogue, or we have to be very carefull.
    // Timestamp We don't need it, just important that the order does not change. Chronological order
    // The way it will be payed out is dependent on the CatalogueTable, or on other table, so this is not dependent on User. This is independent, each song has it's own table.
}


impl Action {
    pub fn to_policy_label(&self) -> String {
        format!("{:?}", self)
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
