use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::types::*;
use std::cmp::min;
use std::ops::Bound;

use crate::*;

/// This is format of output via JSON for the proposal.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalOutput {
    /// Id of the proposal.
    pub id: u64,
    #[serde(flatten)]
    pub proposal: Proposal,
}

/// This is format of output via JSON for the bounty.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct BountyOutput {
    /// Id of the bounty.
    pub id: u64,
    #[serde(flatten)]
    pub bounty: Bounty,
}

#[near_bindgen]
impl Contract {
    /// Returns semver of this contract.
    pub fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// Returns config of this contract.
    pub fn get_config(&self) -> Config {
        self.config.get().unwrap().clone()
    }

    /// Returns policy of this contract.
    pub fn get_policy(&self) -> Policy {
        self.policy.get().unwrap().to_policy().clone()
    }

    /// Returns staking contract if available. Otherwise returns empty.
    pub fn get_staking_contract(self) -> String {
        self.staking_id.map(String::from).unwrap_or_default()
    }

    /// Returns if blob with given hash is stored.
    pub fn has_blob(&self, hash: Base58CryptoHash) -> bool {
        env::storage_has_key(&CryptoHash::from(hash))
    }

    /// Returns locked amount of NEAR that is used for storage.
    pub fn get_locked_storage_amount(&self) -> U128 {
        let locked_storage_amount = env::storage_byte_cost() * (env::storage_usage() as u128);
        U128(locked_storage_amount)
    }

    /// Returns available amount of NEAR that can be spent (outside of amount for storage and bonds).
    pub fn get_available_amount(&self) -> U128 {
        U128(env::account_balance() - self.get_locked_storage_amount().0 - self.locked_amount)
    }

    /// Returns total delegated stake.
    pub fn delegation_total_supply(&self) -> U128 {
        U128(self.total_delegation_amount)
    }

    /// Returns delegated stake to given account.
    pub fn delegation_balance_of(&self, account_id: AccountId) -> U128 {
        U128(self.delegations.get(&account_id).unwrap_or_default())
    }

    /// Combines balance and total amount for calling from external contracts.
    pub fn delegation_balance_ratio(&self, account_id: AccountId) -> (U128, U128) {
        (
            self.delegation_balance_of(account_id),
            self.delegation_total_supply(),
        )
    }

    /// Last proposal's id.
    pub fn get_last_proposal_id(&self) -> u64 {
        self.last_proposal_id
    }

    /// Get proposals in paginated view.
    pub fn get_proposals(&self, from_index: u64, limit: u64) -> Vec<ProposalOutput> {
        (from_index..min(self.last_proposal_id, from_index + limit))
            .filter_map(|id| {
                self.proposals.get(&id).map(|proposal| ProposalOutput {
                    id,
                    proposal: proposal.into(),
                })
            })
            .collect()
    }

    /// List all InProgressNfts
    pub fn get_in_progress_nfts(&self) -> Vec<InProgressMetadata> {
        (0..self.in_progress_nonce)
            .filter_map(|id| {
                self.in_progress_nfts.get(&id)
            })
            .collect()
    }

    /// List CatalogueEntries for an artist's catalogue
    pub fn get_catalogue(&self, artist: AccountId) -> Vec<(TreeIndex, Option<CatalogueEntry>)> {
        let catalogue_for_artist = self.catalogues.get(&artist).unwrap();
        catalogue_for_artist.to_vec()
    }

    /// Get slice of the Catalogue of an Artist
    pub fn get_catalogue_slice(&self, artist: AccountId, from_index: u64, limit: u64) -> Vec<(TreeIndex, Option<CatalogueEntry>)> {
        let catalogue_for_artist = self.catalogues.get(&artist).unwrap();
        let catalogue_as_vec = catalogue_for_artist.to_vec();

        let start: usize = from_index as usize;
        let end: usize = min(catalogue_for_artist.len(), from_index + limit) as usize;

        catalogue_as_vec[start .. end].to_vec()
    }

    /// Get specific Catalogue entries for Artist
    pub fn get_catalogue_entries(&self, artist: AccountId, list: Vec<TreeIndex>) -> Vec<(TreeIndex, Option<CatalogueEntry>)> {
        let catalogue_for_artist = self.catalogues.get(&artist).unwrap();
        let mut result_vec = Vec::new();

        for index in list {
            result_vec.push((
                index,
                catalogue_for_artist.get(&index).unwrap()
            ));
        }

        result_vec
    }

    /// Get a single IncomeTable
    pub fn get_single_income_table(&self, id: TreeIndex) -> IncomeTable {
        self.income_tables.get(&id).unwrap()
    }

    /// Get price for single NFT
    pub fn get_price(&self, minting_contract: AccountId, root_id: TokenId) -> Option<SalePriceInYoctoNear> {
        let uniq_id = UniqId::new(minting_contract, root_id);
        let tree_index = self.uniq_id_to_tree_index.get(&uniq_id.clone()).unwrap();
        
        self.income_tables.get(&tree_index)
        .unwrap()
        .price
    }


    /// List income tables, with possible limit
    pub fn get_income_tables(&self, from_index: u64, limit: u64) -> Vec<(TreeIndex, IncomeTable)> {
        let end = from_index + limit;
        self.income_tables.range((Bound::Included(from_index), Bound::Excluded(end)))
            .collect::<Vec<(TreeIndex, IncomeTable)>>()
    }

    /// Get TreeIndex from MintingContract and RootID, which are the components of UniqId
    pub fn get_tree_index(&self, minting_contract: AccountId, root_id: TokenId) -> Option<TreeIndex> {
        let uniq_id = UniqId::new(minting_contract, root_id);
        self.uniq_id_to_tree_index.get(&uniq_id)
    }

    /// Get the whole UniqId -> TreeIndex map
    pub fn get_all_tree_index(&self) -> Vec<(UniqId, TreeIndex)> {
        self.uniq_id_to_tree_index.to_vec()
    }

    /// Get slice of the UniqId -> TreeIndex mapping
    pub fn get_tree_index_slice(&self, from_index: u64, limit: u64) -> Vec<(UniqId, TreeIndex)> {
        let id_to_index_as_vec = self.uniq_id_to_tree_index.to_vec();

        let start: usize = from_index as usize;
        let end: usize = min(id_to_index_as_vec.len(), (from_index + limit) as usize) as usize;

        id_to_index_as_vec[start .. end].to_vec()
    }

    /// Get number of NFTs that are registered with the DAO, this is equal to the current TreeIndex nonce
    pub fn get_number_of_nfts(&self) -> TreeIndex {
        self.tree_index
    }

    /// List income tables, with possible limit
    pub fn get_failed_transactions(&self, from_index: u64, limit: u64) -> Vec<(u64, FailedTransaction)> {
        let failed_transactions_as_vec = self.failed_transactions.to_vec();

        let start: usize = from_index as usize;
        let end: usize = min(self.failed_transactions.len(), from_index + limit) as usize;

        failed_transactions_as_vec[start .. end].to_vec()
    }

    /// Get specific proposal.
    pub fn get_proposal(&self, id: u64) -> ProposalOutput {
        let proposal = self.proposals.get(&id).expect("ERR_NO_PROPOSAL");
        ProposalOutput {
            id,
            proposal: proposal.into(),
        }
    }

    /// Get given bounty by id.
    pub fn get_bounty(&self, id: u64) -> BountyOutput {
        let bounty = self.bounties.get(&id).expect("ERR_NO_BOUNTY");
        BountyOutput {
            id,
            bounty: bounty.into(),
        }
    }

    /// Get number of bounties.
    pub fn get_last_bounty_id(&self) -> u64 {
        self.last_bounty_id
    }

    /// Get `limit` of bounties from given index.
    pub fn get_bounties(&self, from_index: u64, limit: u64) -> Vec<BountyOutput> {
        (from_index..std::cmp::min(from_index + limit, self.last_bounty_id))
            .filter_map(|id| {
                self.bounties.get(&id).map(|bounty| BountyOutput {
                    id,
                    bounty: bounty.into(),
                })
            })
            .collect()
    }

    /// Get bounty claims for given user.
    pub fn get_bounty_claims(&self, account_id: AccountId) -> Vec<BountyClaim> {
        self.bounty_claimers.get(&account_id).unwrap_or_default()
    }

    /// Returns number of claims per given bounty.
    pub fn get_bounty_number_of_claims(&self, id: u64) -> u32 {
        self.bounty_claims_count.get(&id).unwrap_or_default()
    }
}
