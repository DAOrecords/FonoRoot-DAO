use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, TreeMap};
use near_sdk::json_types::{Base58CryptoHash, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    log, env, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey, CryptoHash,
    PanicOnDefault, Promise, PromiseResult,
};

pub use crate::bounties::{Bounty, BountyClaim, VersionedBounty};
pub use crate::policy::{
    default_policy, Policy, RoleKind, RolePermission, VersionedPolicy, VotePolicy,
};
use crate::proposals::VersionedProposal;
pub use crate::proposals::{Proposal, ProposalInput, ProposalKind, ProposalStatus};
pub use crate::types::*;
use crate::upgrade::{internal_get_factory_info, internal_set_factory_info, FactoryInfo};
pub use crate::views::{BountyOutput, ProposalOutput};

mod bounties;
mod delegation;
mod policy;
mod proposals;
mod types;
mod upgrade;
pub mod views;

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Config,
    Policy,
    Delegations,
    Proposals,
    Bounties,
    BountyClaimers,
    BountyClaimCounts,
    Blobs,
    Catalogues,
    InProgressNfts
}

/// After payouts, allows a callback
#[ext_contract(ext_self)]
pub trait ExtSelf {
    /// Callback after proposal execution.
    fn on_proposal_callback(&mut self, proposal_id: u64) -> PromiseOrValue<()>;
    fn mint_root_callback(&mut self, #[callback_result] result: Result<MintRootResult, near_sdk::PromiseError>, custom: AccountId);
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    /// DAO configuration.
    pub config: LazyOption<Config>,
    /// Voting and permissions policy.
    pub policy: LazyOption<VersionedPolicy>,
    
    /// Amount of $NEAR locked for bonds.
    pub locked_amount: Balance,

    /// Vote staking contract id. That contract must have this account as owner.
    pub staking_id: Option<AccountId>,
    /// Delegated  token total amount.
    pub total_delegation_amount: Balance,
    /// Delegations per user.
    pub delegations: LookupMap<AccountId, Balance>,

    /// Last available id for the proposals.
    pub last_proposal_id: u64,
    /// Proposal map from ID to proposal information.
    pub proposals: LookupMap<u64, VersionedProposal>,

    /// Last available id for the bounty.
    pub last_bounty_id: u64,
    /// Bounties map from ID to bounty information.
    pub bounties: LookupMap<u64, VersionedBounty>,
    /// Bounty claimers map per user. Allows quickly to query for each users their claims.
    pub bounty_claimers: LookupMap<AccountId, Vec<BountyClaim>>,
    /// Count of claims per bounty.
    pub bounty_claims_count: LookupMap<u64, u32>,

    /// Large blob storage.
    pub blobs: LookupMap<CryptoHash, AccountId>,

    // **TODO** Implement `pending_nfts` here.  A prepairer-place
    // We need to know if LookupMap can handle deleting entries from the middle (in_progress_nonce has to go up, we are not repeating IDs)
    // I think if we use in_progress_nfts.remove(5), when we are doing the actual minting, that way we well get the data in a way that it is also deleted
    // from this list, IF the transaction goes through, otherwise it will not change
    pub in_progress_nfts: LookupMap<u64, InProgressMetadata>,

    // **TODO** Implement in_progress_nonce here
    pub in_progress_nonce: u64,

    // **TODO** Implement Catalogues here
    pub catalogues: LookupMap<AccountId, Catalogue>,

    // **TODO** Implement Income Tables
    pub income_tables:  TreeMap<TreeIndex, IncomeTable>,

    // **TODO** We might do this, but not sure yet. The original problem is, Key in TreeMap can't be string.
    // This way we would have an iterable, ordered collection of all the NFTs, but we couldn't point to an element by contract+root_id, but with this helper, we could
    // We could call it uniq_id_to_tree_index
    pub uniq_id_to_tree_index: UnorderedMap<UniqId, TreeIndex>,           //UniqIdStore

    // **TODO** It gets even more complicated, because we need a nonce as well.
    pub tree_index: TreeIndex,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(config: Config, policy: VersionedPolicy) -> Self {
        let this = Self {
            config: LazyOption::new(StorageKeys::Config, Some(&config)),
            policy: LazyOption::new(StorageKeys::Policy, Some(&policy.upgrade())),
            staking_id: None,
            total_delegation_amount: 0,
            delegations: LookupMap::new(StorageKeys::Delegations),
            last_proposal_id: 0,
            proposals: LookupMap::new(StorageKeys::Proposals),
            last_bounty_id: 0,
            bounties: LookupMap::new(StorageKeys::Bounties),
            bounty_claimers: LookupMap::new(StorageKeys::BountyClaimers),
            bounty_claims_count: LookupMap::new(StorageKeys::BountyClaimCounts),
            blobs: LookupMap::new(StorageKeys::Blobs),
            locked_amount: 0,
            in_progress_nfts: LookupMap::new(StorageKeys::InProgressNfts),
            in_progress_nonce: 0,
            catalogues: LookupMap::new(StorageKeys::Catalogues),
            income_tables: TreeMap::new(b"t"),
            uniq_id_to_tree_index: UnorderedMap::new(b"m"),
            tree_index: 0,
        };
        internal_set_factory_info(&FactoryInfo {
            factory_id: env::predecessor_account_id(),
            auto_update: true,
        });
        this
    }

    /// Should only be called by this contract on migration.
    /// This is NOOP implementation. KEEP IT if you haven't changed contract state.
    /// If you have changed state, you need to implement migration from old state (keep the old struct with different name to deserialize it first).
    /// After migrate goes live on MainNet, return this implementation for next updates.
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let this: Contract = env::state_read().expect("ERR_CONTRACT_IS_NOT_INITIALIZED");
        this
    }

    /// Remove blob from contract storage and pay back to original storer.
    /// Only original storer can call this.
    pub fn remove_blob(&mut self, hash: Base58CryptoHash) -> Promise {
        let hash: CryptoHash = hash.into();
        let account_id = self.blobs.remove(&hash).expect("ERR_NO_BLOB");
        assert_eq!(
            env::predecessor_account_id(),
            account_id,
            "ERR_INVALID_CALLER"
        );
        env::storage_remove(&hash);
        let blob_len = env::register_len(u64::MAX - 1).unwrap();
        let storage_cost = ((blob_len + 32) as u128) * env::storage_byte_cost();
        Promise::new(account_id).transfer(storage_cost)
    }

    /// Returns factory information, including if auto update is allowed.
    pub fn get_factory_info(&self) -> FactoryInfo {
        internal_get_factory_info()
    }

    /// A very simple test that will show us whether we are ready to work with Satori or not
    pub fn satori_test() {
        // 1 . Do a Cross-Contract-Call to the Satori contract
        // 2 . Get result
        // 3 . From the result, read the field that we need
        // 4. Log the result of the test
        //    -> "Ok"
        //    -> "Unauthorized"
    }

    /// Callback for MintRoot, this function will create the IncomeTable entry
    #[private]
    pub fn mint_root_callback(
        &mut self, 
        #[callback_result] result: Result<MintRootResult, near_sdk::PromiseError>,
        custom: AccountId   // most likely custom could be renamed to anything. It's safer to use this instead of env::signer_account_id()
    ) {
        // If the entry exists in IncomeTable, that means that the NFT exists. 
        // This is the single-point-of-truth, IncomeTable could be iterated for example to get a list of NFTs

        if result.is_err() {
            env::log_str("Error.");
            //panic_str!("Error.");
            //false
        }

        let mint_root_result: MintRootResult = result.unwrap();

        log!("Mint Root Callback. All good.");
        log!("Custom: {:?}", custom.clone());
        log!("Contract: {:?}", mint_root_result.contract);
        log!("RootID: {:?}", mint_root_result.root_id);
        log!("Predecessor account ID: {:?}", env::predecessor_account_id());
        log!("Signer account ID: {:?}", env::signer_account_id());
        let uniq_id = format!("{}-{}", mint_root_result.contract, mint_root_result.root_id);
        log!("Uniq ID: {:?}", uniq_id);

        let new_income_table = IncomeTable {
            total_income: 0,
            current_balance: 0,
            root_id: mint_root_result.root_id,
            contract: mint_root_result.contract,
            owner: custom.clone(),
        };

        // !! Be carefull not to wipe the total_income by somehow tricking the contract into believing that this is a new RootNFT, when it isn't!
        //UniqId::new(mint_root_result.contract, mint_root_result.root_id);

        self.uniq_id_to_tree_index.insert(&uniq_id, &self.tree_index);
        self.income_tables.insert(&self.tree_index, &new_income_table);
        
        // Get existing catalogue for artist, or create a new one
        let mut catalogue_for_owner = self.catalogues.get(&custom).unwrap_or_else(|| -> Catalogue {
            Catalogue::new(b"m")
            // probably it would be good if we wouldn't repeat this init param
        });
        log!("Catalogue for owner: {:?}", catalogue_for_owner);
        // Insert an empty entry for the newly minted song
        catalogue_for_owner.insert(&self.tree_index, &None);
        log!("Catalogue for owner after insert: {:?}", catalogue_for_owner);
        // Insert back the catalogue to the artist
        self.catalogues.insert(&custom, &catalogue_for_owner);
        
        self.tree_index = self.tree_index + 1;
        //true
    }
}

/// Stores attached data into blob store and returns hash of it.
/// Implemented to avoid loading the data into WASM for optimal gas usage.
#[no_mangle]
pub extern "C" fn store_blob() {
    env::setup_panic_hook();
    let mut contract: Contract = env::state_read().expect("ERR_CONTRACT_IS_NOT_INITIALIZED");
    let input = env::input().expect("ERR_NO_INPUT");
    let sha256_hash = env::sha256(&input);
    assert!(!env::storage_has_key(&sha256_hash), "ERR_ALREADY_EXISTS");

    let blob_len = input.len();
    let storage_cost = ((blob_len + 32) as u128) * env::storage_byte_cost();
    assert!(
        env::attached_deposit() >= storage_cost,
        "ERR_NOT_ENOUGH_DEPOSIT:{}",
        storage_cost
    );

    env::storage_write(&sha256_hash, &input);
    let mut blob_hash = [0u8; 32];
    blob_hash.copy_from_slice(&sha256_hash);
    contract
        .blobs
        .insert(&blob_hash, &env::predecessor_account_id());
    let blob_hash_str = near_sdk::serde_json::to_string(&Base58CryptoHash::from(blob_hash))
        .unwrap()
        .into_bytes();

    env::value_return(&blob_hash_str);
    env::state_write(&contract);
}

#[cfg(test)]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use near_sdk_sim::to_yocto;

    use crate::proposals::ProposalStatus;

    use super::*;

    fn create_proposal(context: &mut VMContextBuilder, contract: &mut Contract) -> u64 {
        testing_env!(context.attached_deposit(to_yocto("1")).build());
        contract.add_proposal(ProposalInput {
            description: "test".to_string(),
            kind: ProposalKind::Transfer {
                token_id: String::from(OLD_BASE_TOKEN),
                receiver_id: accounts(2).into(),
                amount: U128(to_yocto("100")),
                msg: None,
            },
        })
    }

    #[test]
    fn test_basics() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(1)).build());
        let mut contract = Contract::new(
            Config::test_config(),
            VersionedPolicy::Default(vec![accounts(1).into()]),
        );
        let id = create_proposal(&mut context, &mut contract);
        assert_eq!(contract.get_proposal(id).proposal.description, "test");
        assert_eq!(contract.get_proposals(0, 10).len(), 1);

        let id = create_proposal(&mut context, &mut contract);
        contract.act_proposal(id, Action::VoteApprove, None);
        assert_eq!(
            contract.get_proposal(id).proposal.status,
            ProposalStatus::Approved
        );

        let id = create_proposal(&mut context, &mut contract);
        // proposal expired, finalize.
        testing_env!(context
            .block_timestamp(1_000_000_000 * 24 * 60 * 60 * 8)
            .build());
        contract.act_proposal(id, Action::Finalize, None);
        assert_eq!(
            contract.get_proposal(id).proposal.status,
            ProposalStatus::Expired
        );

        // non council adding proposal per default policy.
        testing_env!(context
            .predecessor_account_id(accounts(2))
            .attached_deposit(to_yocto("1"))
            .build());
        let _id = contract.add_proposal(ProposalInput {
            description: "test".to_string(),
            kind: ProposalKind::AddMemberToRole {
                member_id: accounts(2).into(),
                role: "council".to_string(),
            },
        });
    }

    #[test]
    #[should_panic(expected = "ERR_PERMISSION_DENIED")]
    fn test_remove_proposal_denied() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(1)).build());
        let mut contract = Contract::new(
            Config::test_config(),
            VersionedPolicy::Default(vec![accounts(1).into()]),
        );
        let id = create_proposal(&mut context, &mut contract);
        assert_eq!(contract.get_proposal(id).proposal.description, "test");
        contract.act_proposal(id, Action::RemoveProposal, None);
    }

    #[test]
    fn test_remove_proposal_allowed() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(1)).build());
        let mut policy = VersionedPolicy::Default(vec![accounts(1).into()]).upgrade();
        policy.to_policy_mut().roles[1]
            .permissions
            .insert("*:RemoveProposal".to_string());
        let mut contract = Contract::new(Config::test_config(), policy);
        let id = create_proposal(&mut context, &mut contract);
        assert_eq!(contract.get_proposal(id).proposal.description, "test");
        contract.act_proposal(id, Action::RemoveProposal, None);
        assert_eq!(contract.get_proposals(0, 10).len(), 0);
    }

    #[test]
    fn test_vote_expired_proposal() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(1)).build());
        let mut contract = Contract::new(
            Config::test_config(),
            VersionedPolicy::Default(vec![accounts(1).into()]),
        );
        let id = create_proposal(&mut context, &mut contract);
        testing_env!(context
            .block_timestamp(1_000_000_000 * 24 * 60 * 60 * 8)
            .build());
        contract.act_proposal(id, Action::VoteApprove, None);
    }

    #[test]
    #[should_panic(expected = "ERR_ALREADY_VOTED")]
    fn test_vote_twice() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(1)).build());
        // **TODO** Most likely this is also failing because we've changed the DefaultVotePolicy
        let mut contract = Contract::new(
            Config::test_config(),
            VersionedPolicy::Default(vec![accounts(1).into(), accounts(2).into()]),
        );
        let id = create_proposal(&mut context, &mut contract);
        contract.act_proposal(id, Action::VoteApprove, None);
        contract.act_proposal(id, Action::VoteApprove, None);
    }

    #[test]
    fn test_add_to_missing_role() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(1)).build());
        let mut contract = Contract::new(
            Config::test_config(),
            VersionedPolicy::Default(vec![accounts(1).into()]),
        );
        testing_env!(context.attached_deposit(to_yocto("1")).build());
        let id = contract.add_proposal(ProposalInput {
            description: "test".to_string(),
            kind: ProposalKind::AddMemberToRole {
                member_id: accounts(2).into(),
                role: "missing".to_string(),
            },
        });
        contract.act_proposal(id, Action::VoteApprove, None);
        let x = contract.get_policy();
        // still 2 roles: all and council.
        assert_eq!(x.roles.len(), 2);
    }

    #[test]
    #[should_panic(expected = "ERR_INVALID_POLICY")]
    fn test_fails_adding_invalid_policy() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(1)).build());
        let mut contract = Contract::new(
            Config::test_config(),
            VersionedPolicy::Default(vec![accounts(1).into()]),
        );
        testing_env!(context.attached_deposit(to_yocto("1")).build());
        let _id = contract.add_proposal(ProposalInput {
            description: "test".to_string(),
            kind: ProposalKind::ChangePolicy {
                policy: VersionedPolicy::Default(vec![]),
            },
        });
    }
}
