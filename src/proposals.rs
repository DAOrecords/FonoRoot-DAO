use std::collections::HashMap;

use near_contract_standards::fungible_token::core_impl::ext_fungible_token;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base64VecU8, U128, U64};
use near_sdk::{log, AccountId, Balance, Gas, PromiseOrValue};

use crate::policy::UserInfo;
use crate::types::{
    convert_old_to_new_token, Action, Config, OldAccountId, GAS_FOR_FT_TRANSFER, OLD_BASE_TOKEN,
    ONE_YOCTO_NEAR, WeDontKnow, NftDataFromFrontEnd, MintingContractArgs, MintingContractMeta, MintingContractExtra,
    RevenueTable, SalePriceInYoctoNear, TokenId
};
use crate::upgrade::{upgrade_remote, upgrade_using_factory};
use crate::*;

/// Status of a proposal.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalStatus {
    InProgress,
    /// If quorum voted yes, this proposal is successfully approved.
    Approved,
    /// If quorum voted no, this proposal is rejected. Bond is returned.
    Rejected,
    /// If quorum voted to remove (e.g. spam), this proposal is rejected and bond is not returned.
    /// Interfaces shouldn't show removed proposals.
    Removed,
    /// Expired after period of time.
    Expired,
    /// If proposal was moved to Hub or somewhere else.
    Moved,
    /// If proposal has failed when finalizing. Allowed to re-finalize again to either expire or approved.
    Failed,
}

/// Function call arguments.
// **WARNING** Everything was rewritten as pub
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct ActionCall {
    pub method_name: String,
    pub args: Base64VecU8,
    pub deposit: U128,
    pub gas: U64,
}

/// Function call arguments.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct PolicyParameters {
    pub proposal_bond: Option<U128>,
    pub proposal_period: Option<U64>,
    pub bounty_bond: Option<U128>,
    pub bounty_forgiveness_period: Option<U64>,
}

/// Kinds of proposals, doing different action.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalKind {
    /// Change the DAO config.
    ChangeConfig { config: Config },
    /// Change the full policy.
    ChangePolicy { policy: VersionedPolicy },
    /// Add member to given role in the policy. This is short cut to updating the whole policy.
    AddMemberToRole { member_id: AccountId, role: String },
    /// Remove member to given role in the policy. This is short cut to updating the whole policy.
    RemoveMemberFromRole { member_id: AccountId, role: String },
    /// Calls `receiver_id` with list of method names in a single promise.
    /// Allows this contract to execute any arbitrary set of actions in other contracts.
    FunctionCall {
        receiver_id: AccountId,
        actions: Vec<ActionCall>,
    },
    /// Upgrade this contract with given hash from blob store.
    UpgradeSelf { hash: Base58CryptoHash },
    /// Upgrade another contract, by calling method with the code from given hash from blob store.
    UpgradeRemote {
        receiver_id: AccountId,
        method_name: String,
        hash: Base58CryptoHash,
    },
    /// Transfers given amount of `token_id` from this DAO to `receiver_id`.
    /// If `msg` is not None, calls `ft_transfer_call` with given `msg`. Fails if this base token.
    /// For `ft_transfer` and `ft_transfer_call` `memo` is the `description` of the proposal.
    Transfer {
        /// Can be "" for $NEAR or a valid account id.
        token_id: OldAccountId,
        receiver_id: AccountId,
        amount: U128,
        msg: Option<String>,
    },
    /// Sets staking contract. Can only be proposed if staking contract is not set yet.
    SetStakingContract { staking_id: AccountId },
    /// Add new bounty.
    AddBounty { bounty: Bounty },
    /// Indicates that given bounty is done by given user.
    BountyDone {
        bounty_id: u64,
        receiver_id: AccountId,
    },
    /// Just a signaling vote, with no execution.
    Vote,
    /// Change information about factory and auto update.
    FactoryInfoUpdate { factory_info: FactoryInfo },
    /// Add new role to the policy. If the role already exists, update it. This is short cut to updating the whole policy.
    ChangePolicyAddOrUpdateRole { role: RolePermission },
    /// Remove role from the policy. This is short cut to updating the whole policy.
    ChangePolicyRemoveRole { role: String },
    /// Update the default vote policy from the policy. This is short cut to updating the whole policy.
    ChangePolicyUpdateDefaultVotePolicy { vote_policy: VotePolicy },
    /// Update the parameters from the policy. This is short cut to updating the whole policy.
    ChangePolicyUpdateParameters { parameters: PolicyParameters },
    // **TODO** Add MintRoot and the other necesarry functions here
    MintRoot { id: u64 },
    PrepairNft { nft_data: NftDataFromFrontEnd },
    UpdatePrepairedNft { id: u64, new_nft_data: NftDataFromFrontEnd },
    CreateRevenueTable { id: TokenId,  contract: AccountId, unsafe_table: HashMap<AccountId, u64>, price: SalePriceInYoctoNear },
    AlterRevenueTable { tree_index: TreeIndex, unsafe_table: HashMap<AccountId, u64>, price: SalePriceInYoctoNear }, 
    ScheduleMint { params: WeDontKnow },
}

impl ProposalKind {
    /// Returns label of policy for given type of proposal.
    pub fn to_policy_label(&self) -> &str {
        match self {
            ProposalKind::ChangeConfig { .. } => "config",
            ProposalKind::ChangePolicy { .. } => "policy",
            ProposalKind::AddMemberToRole { .. } => "add_member_to_role",
            ProposalKind::RemoveMemberFromRole { .. } => "remove_member_from_role",
            ProposalKind::FunctionCall { .. } => "call",
            ProposalKind::UpgradeSelf { .. } => "upgrade_self",
            ProposalKind::UpgradeRemote { .. } => "upgrade_remote",
            ProposalKind::Transfer { .. } => "transfer",
            ProposalKind::SetStakingContract { .. } => "set_vote_token",
            ProposalKind::AddBounty { .. } => "add_bounty",
            ProposalKind::BountyDone { .. } => "bounty_done",
            ProposalKind::Vote => "vote",
            ProposalKind::FactoryInfoUpdate { .. } => "factory_info_update",
            ProposalKind::ChangePolicyAddOrUpdateRole { .. } => "policy_add_or_update_role",
            ProposalKind::ChangePolicyRemoveRole { .. } => "policy_remove_role",
            ProposalKind::ChangePolicyUpdateDefaultVotePolicy { .. } => {
                "policy_update_default_vote_policy"
            }
            ProposalKind::ChangePolicyUpdateParameters { .. } => "policy_update_parameters",
            // **TODO** Add the same functions here as above
            ProposalKind::MintRoot { .. } => "mint_root",
            ProposalKind::PrepairNft { .. } => "prepair_nft",
            ProposalKind::UpdatePrepairedNft { .. } => "update_prepaired_nft",
            ProposalKind::CreateRevenueTable { .. } => "create_revenue_table",
            ProposalKind::AlterRevenueTable { .. }  => "alter_revenue_table",
            ProposalKind::ScheduleMint { .. } => "schedule_mint"
        }
    }
}

/// Votes recorded in the proposal.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Vote {
    Approve = 0x0,
    Reject = 0x1,
    Remove = 0x2,
}

impl From<Action> for Vote {
    fn from(action: Action) -> Self {
        match action {
            Action::VoteApprove => Vote::Approve,
            Action::VoteReject => Vote::Reject,
            Action::VoteRemove => Vote::Remove,
            _ => unreachable!(),
        }
    }
}

/// Proposal that are sent to this DAO.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    /// Original proposer.
    pub proposer: AccountId,
    /// Description of this proposal.
    pub description: String,
    /// Kind of proposal with relevant information.
    pub kind: ProposalKind,
    /// Current status of the proposal.
    pub status: ProposalStatus,
    /// Count of votes per role per decision: yes / no / spam.
    pub vote_counts: HashMap<String, [Balance; 3]>,
    /// Map of who voted and how.
    pub votes: HashMap<AccountId, Vote>,
    /// Submission time (for voting period).
    pub submission_time: U64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum VersionedProposal {
    Default(Proposal),
}

impl From<VersionedProposal> for Proposal {
    fn from(v: VersionedProposal) -> Self {
        match v {
            VersionedProposal::Default(p) => p,
        }
    }
}

impl Proposal {
    /// Adds vote of the given user with given `amount` of weight. If user already voted, fails.
    pub fn update_votes(
        &mut self,
        account_id: &AccountId,
        roles: &[String],
        vote: Vote,
        policy: &Policy,
        user_weight: Balance,
    ) {
        for role in roles {
            let amount = if policy.is_token_weighted(role, &self.kind.to_policy_label().to_string())
            {
                user_weight
            } else {
                1
            };
            self.vote_counts.entry(role.clone()).or_insert([0u128; 3])[vote.clone() as usize] +=
                amount;
        }
        assert!(
            self.votes.insert(account_id.clone(), vote).is_none(),
            "ERR_ALREADY_VOTED"
        );
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalInput {
    /// Description of this proposal.
    pub description: String,
    /// Kind of proposal with relevant information.
    pub kind: ProposalKind,
}

impl From<ProposalInput> for Proposal {
    fn from(input: ProposalInput) -> Self {
        Self {
            proposer: env::predecessor_account_id(),
            description: input.description,
            kind: input.kind,
            status: ProposalStatus::InProgress,
            vote_counts: HashMap::default(),
            votes: HashMap::default(),
            submission_time: U64::from(env::block_timestamp()),
        }
    }
}

impl Contract {
    /// Execute payout of given token to given user.
    pub(crate) fn internal_payout(
        &mut self,
        token_id: &Option<AccountId>,
        receiver_id: &AccountId,
        amount: Balance,
        memo: String,
        msg: Option<String>,
    ) -> PromiseOrValue<()> {
        if token_id.is_none() {
            Promise::new(receiver_id.clone()).transfer(amount).into()
        } else {
            if let Some(msg) = msg {
                ext_fungible_token::ft_transfer_call(
                    receiver_id.clone(),
                    U128(amount),
                    Some(memo),
                    msg,
                    token_id.as_ref().unwrap().clone(),
                    ONE_YOCTO_NEAR,
                    GAS_FOR_FT_TRANSFER,
                )
            } else {
                ext_fungible_token::ft_transfer(
                    receiver_id.clone(),
                    U128(amount),
                    Some(memo),
                    token_id.as_ref().unwrap().clone(),
                    ONE_YOCTO_NEAR,
                    GAS_FOR_FT_TRANSFER,
                )
            }
            .into()
        }
    }

    fn internal_return_bonds(&mut self, policy: &Policy, proposal: &Proposal) -> Promise {
        match &proposal.kind {
            ProposalKind::BountyDone { .. } => {
                self.locked_amount -= policy.bounty_bond.0;
                Promise::new(proposal.proposer.clone()).transfer(policy.bounty_bond.0);
            }
            _ => {}
        }

        self.locked_amount -= policy.proposal_bond.0;
        Promise::new(proposal.proposer.clone()).transfer(policy.proposal_bond.0)
    }

    /// Executes given proposal and updates the contract's state.
    fn internal_execute_proposal(
        &mut self,
        policy: &Policy,
        proposal: &Proposal,
        proposal_id: u64,
    ) -> PromiseOrValue<()> {
        let result = match &proposal.kind {
            ProposalKind::ChangeConfig { config } => {
                self.config.set(config);
                PromiseOrValue::Value(())
            }
            ProposalKind::ChangePolicy { policy } => {
                self.policy.set(policy);
                PromiseOrValue::Value(())
            }
            ProposalKind::AddMemberToRole { member_id, role } => {
                let mut new_policy = policy.clone();
                new_policy.add_member_to_role(role, &member_id.clone().into());
                self.policy.set(&VersionedPolicy::Current(new_policy));
                PromiseOrValue::Value(())
            }
            ProposalKind::RemoveMemberFromRole { member_id, role } => {
                let mut new_policy = policy.clone();
                new_policy.remove_member_from_role(role, &member_id.clone().into());
                self.policy.set(&VersionedPolicy::Current(new_policy));
                PromiseOrValue::Value(())
            }
            ProposalKind::FunctionCall {
                receiver_id,
                actions,
            } => {
                let mut promise = Promise::new(receiver_id.clone().into());
                for action in actions {
                    promise = promise.function_call(
                        action.method_name.clone().into(),
                        action.args.clone().into(),
                        action.deposit.0,
                        Gas(action.gas.0),
                    )
                }
                promise.into()
            }
            ProposalKind::UpgradeSelf { hash } => {
                upgrade_using_factory(hash.clone());
                PromiseOrValue::Value(())
            }
            ProposalKind::UpgradeRemote {
                receiver_id,
                method_name,
                hash,
            } => {
                upgrade_remote(&receiver_id, method_name, &CryptoHash::from(hash.clone()));
                PromiseOrValue::Value(())
            }
            ProposalKind::Transfer {
                token_id,
                receiver_id,
                amount,
                msg,
            } => self.internal_payout(
                &convert_old_to_new_token(token_id),
                &receiver_id,
                amount.0,
                proposal.description.clone(),
                msg.clone(),
            ),
            ProposalKind::SetStakingContract { staking_id } => {
                assert!(self.staking_id.is_none(), "ERR_INVALID_STAKING_CHANGE");
                self.staking_id = Some(staking_id.clone().into());
                PromiseOrValue::Value(())
            }
            ProposalKind::AddBounty { bounty } => {
                self.internal_add_bounty(bounty);
                PromiseOrValue::Value(())
            }
            ProposalKind::BountyDone {
                bounty_id,
                receiver_id,
            } => self.internal_execute_bounty_payout(*bounty_id, &receiver_id.clone().into(), true),
            ProposalKind::Vote => PromiseOrValue::Value(()),
            ProposalKind::FactoryInfoUpdate { factory_info } => {
                internal_set_factory_info(factory_info);
                PromiseOrValue::Value(())
            }
            ProposalKind::ChangePolicyAddOrUpdateRole { role } => {
                let mut new_policy = policy.clone();
                new_policy.add_or_update_role(role);
                self.policy.set(&VersionedPolicy::Current(new_policy));
                PromiseOrValue::Value(())
            }
            ProposalKind::ChangePolicyRemoveRole { role } => {
                let mut new_policy = policy.clone();
                new_policy.remove_role(role);
                self.policy.set(&VersionedPolicy::Current(new_policy));
                PromiseOrValue::Value(())
            }
            ProposalKind::ChangePolicyUpdateDefaultVotePolicy { vote_policy } => {
                let mut new_policy = policy.clone();
                new_policy.update_default_vote_policy(vote_policy);
                self.policy.set(&VersionedPolicy::Current(new_policy));
                PromiseOrValue::Value(())
            }
            ProposalKind::ChangePolicyUpdateParameters { parameters } => {
                let mut new_policy = policy.clone();
                new_policy.update_parameters(parameters);
                self.policy.set(&VersionedPolicy::Current(new_policy));
                PromiseOrValue::Value(())
            }
            ProposalKind::MintRoot { id } => {
                log!("Entering MintRoot...");

                let selected_draft = self.in_progress_nfts.remove(id).unwrap();
                let fonoroot: AccountId =  selected_draft.contract;
                self.assert_artist_can_mint(fonoroot.clone());
                assert_eq!{
                    env::predecessor_account_id(),
                    selected_draft.artist,
                    "Only the owner of the draft can mint!"
                };
                
                let extra = near_sdk::serde_json::to_string( &MintingContractExtra {
                    music_cid: selected_draft.music.unwrap(),
                    music_hash: None,
                    parent: None,
                    instance_nonce: 999_999_999,
                    generation: 999_999_999,
                }).unwrap();

                // Validation should happen at this point. That validation that the param exists is already done by .unwrap()
                // We need to add hashes for `reference`, `media`, and `music_cid`

                let args = MintingContractArgs {
                    receiver_id: selected_draft.artist.clone(),
                    metadata: MintingContractMeta {
                        title: selected_draft.title.unwrap(),
                        description: selected_draft.desc.unwrap(),
                        reference: selected_draft.meta.unwrap(),
                        reference_hash: None,                                     // Will need to do this later
                        media: selected_draft.image.unwrap(),
                        media_hash: None,                                         // Will need to do this later
                        copies: None,                                             // Will be None, we are not using this.
                        issued_at: None,                                          // Will be None, we are not using this.
                        expires_at: None,                                         // Will be None, we are not using this.
                        starts_at: None,                                          // Will be None, we are not using this.
                        updated_at: None,                                         // Will be None, we are not using this.
                        extra: Some(extra)
                    }
                };
                
                let json_args = near_sdk::serde_json::to_string(&args).unwrap();
                let base64_args = json_args.clone().into_bytes();                
                
                let mut promise = Promise::new(fonoroot.clone().into());
                
                let action = ActionCall {
                    method_name: "mint_root".to_string(),
                    args: base64_args.into(),
                    deposit: U128(200000000000000000000000),
                    gas: U64(100000000000000),
                };
                
                log!("Prepairing cross-contract call...");
                
                promise = promise.function_call(
                    action.method_name.clone().into(),
                    action.args.clone().into(),
                    action.deposit.0,
                    Gas(action.gas.0),
                )
                .then(ext_self::mint_root_callback(
                    selected_draft.artist,
                    env::current_account_id(),
                    0,
                    Gas(50000000000000)
                ));
                
                log!("Initiating cross-contract call! Function inside DAO contract exiting...");
                promise.into()
            }
            ProposalKind::PrepairNft { nft_data } => {
                self.assert_artist_can_mint(nft_data.contract.clone());

                let the_new_nft_data = InProgressMetadata {
                    id: self.in_progress_nonce,
                    initiated: env::block_timestamp(),
                    artist: env::predecessor_account_id(),
                    contract: nft_data.contract.clone(),
                    scheduled: None,
                    title: nft_data.title.clone(),
                    desc: nft_data.desc.clone(),
                    image: nft_data.image_cid.clone(),
                    music: nft_data.music_folder_cid.clone(),
                    meta: nft_data.meta_json_cid.clone(),
                };
                
                self.in_progress_nfts.insert(&self.in_progress_nonce, &the_new_nft_data);
                self.in_progress_nonce = self.in_progress_nonce + 1;
                PromiseOrValue::Value(())
            }
            ProposalKind::UpdatePrepairedNft { id, new_nft_data } => {
                self.assert_artist_can_mint(new_nft_data.contract.clone());
                let old_data = self.in_progress_nfts.get(&id).unwrap();

                // By this we also make sure that the user can't insert an item into a LookUpMap to an arbitrary position, for example, after the nonce
                assert_eq!(
                    &old_data.artist,
                    &env::predecessor_account_id(),
                    "You can only update prepaired NFTs that you originally created!"
                );

                let updated_nft_data = InProgressMetadata {
                    id: id.clone(),
                    initiated: old_data.initiated,
                    artist: env::predecessor_account_id(),
                    contract: new_nft_data.contract.clone(),
                    scheduled: old_data.scheduled,
                    title: new_nft_data.title.clone(),
                    desc: new_nft_data.desc.clone(),
                    image: new_nft_data.image_cid.clone(),
                    music: new_nft_data.music_folder_cid.clone(),
                    meta: new_nft_data.meta_json_cid.clone(),
                };

                self.in_progress_nfts.insert(id, &updated_nft_data);

                PromiseOrValue::Value(())
            },
            ProposalKind::CreateRevenueTable { id, contract, unsafe_table, price } => {
                // Create a revenue table for a RootNFT that was already created
                let uniq_id = format!("{}-{}", contract, id);
                /*log!("Length: {}", self.uniq_id_to_tree_index.len());
                for x in self.uniq_id_to_tree_index.values() {
                    log!("Values: {}", x);
                }
                for y in self.uniq_id_to_tree_index.keys() {
                    log!("Keys: {}", y);
                }*/
                let tree_index = self.uniq_id_to_tree_index.get(&uniq_id.clone()).unwrap();
                //let tree_index = 0; There is a very serious error here, but we don't understand what it is ({"index":0,"kind":{"ExecutionError":"Smart contract panicked: Cannot deserialize value with Borsh"})
                let mut income_table = self.income_tables.get(&tree_index.clone()).unwrap();
                
                let revenue_table = RevenueTable::new(unsafe_table.clone()).unwrap();


                log!("Uniq ID: {:?}", uniq_id.clone());
                log!("Tree Index: {:?}", tree_index.clone());
                log!("Income Table: {:?}", income_table.clone());
                log!("Revenue Table from front end: {:?}", revenue_table);
                log!("Price from front end: {:?}", price);
                
                // Prepair Revenue Entry
                let mut catalogue_for_caller = self.catalogues.get(&env::signer_account_id()).unwrap();              // Has to exist. Otherwise, panic.
                
                // Validate that caller has the right to modify this entry / add new entry.
                assert_eq!(
                    income_table.owner,
                    env::signer_account_id(),
                    "Only the owner (Artist) can alter the revenue table!"
                );
                
                // Update is not possible through this proposal
                if catalogue_for_caller.get(&tree_index).is_none() {
                    panic!("A Revenue Table already exists!");
                }
                
                let new_entry = CatalogueEntry {
                    revenue_table: revenue_table.clone(),
                };
                income_table.price = Some(price.clone());
                self.income_tables.insert(&tree_index.clone(), &income_table);
                catalogue_for_caller.insert(&tree_index, &Some(new_entry));
                self.catalogues.insert(&env::signer_account_id(), &catalogue_for_caller);

                PromiseOrValue::Value(())
            },
            ProposalKind::AlterRevenueTable { tree_index, unsafe_table, price } => {
                // Update a revenue table for a RootNFT
                log!("Hello from Alter Revenue Table!");
                log!("TreeIndex: {:?}", tree_index);

                let new_revenue_table = RevenueTable::new(unsafe_table.clone()).unwrap();
                let mut income_table = self.income_tables.get(&tree_index.clone()).unwrap();
                log!("The new Revenue Table from front end: {:?}", new_revenue_table);
                log!("Income Table: {:?}", income_table.clone());
                log!("New price: {:?}", price);

                // Prepair Revenue Entry
                let mut catalogue_for_caller = self.catalogues.get(&env::signer_account_id()).unwrap();              // Has to exist. Otherwise, panic.
                
                // Validate that caller has the right to modify this entry / add new entry.
                assert_eq!(
                    income_table.owner,
                    env::signer_account_id(),
                    "Only the owner (Artist) can alter the revenue table!"
                );

                let new_entry = CatalogueEntry {
                    revenue_table: new_revenue_table.clone(),
                };
                income_table.price = Some(price.clone());

                catalogue_for_caller.insert(&tree_index, &Some(new_entry));
                self.catalogues.insert(&env::signer_account_id(), &catalogue_for_caller);

                log!("New RevenueTable entry was inserted: {:?}", self.catalogues.get(&env::signer_account_id()).unwrap());

                PromiseOrValue::Value(())
            }
            ProposalKind::ScheduleMint { params: _ } => {
                //self.assert_artist_can_mint(nft_data.contract.clone());

                PromiseOrValue::Value(())
            }
        };
        match result {
            PromiseOrValue::Promise(promise) => promise
                .then(ext_self::on_proposal_callback(
                    proposal_id,
                    env::current_account_id(),
                    0,
                    GAS_FOR_FT_TRANSFER,
                ))
                .into(),
            PromiseOrValue::Value(()) => self.internal_return_bonds(&policy, &proposal).into(),
        }
    }

    pub(crate) fn internal_callback_proposal_success(
        &mut self,
        proposal: &mut Proposal,
    ) -> PromiseOrValue<()> {
        let policy = self.policy.get().unwrap().to_policy();
        if let ProposalKind::BountyDone { bounty_id, .. } = proposal.kind {
            let mut bounty: Bounty = self.bounties.get(&bounty_id).expect("ERR_NO_BOUNTY").into();
            if bounty.times == 0 {
                self.bounties.remove(&bounty_id);
            } else {
                bounty.times -= 1;
                self.bounties
                    .insert(&bounty_id, &VersionedBounty::Default(bounty));
            }
        }
        proposal.status = ProposalStatus::Approved;
        self.internal_return_bonds(&policy, &proposal).into()
    }

    pub(crate) fn internal_callback_proposal_fail(
        &mut self,
        proposal: &mut Proposal,
    ) -> PromiseOrValue<()> {
        proposal.status = ProposalStatus::Failed;
        PromiseOrValue::Value(())
    }

    /// Process rejecting proposal.
    fn internal_reject_proposal(
        &mut self,
        policy: &Policy,
        proposal: &Proposal,
        return_bonds: bool,
    ) -> PromiseOrValue<()> {
        if return_bonds {
            // Return bond to the proposer.
            self.internal_return_bonds(policy, proposal);
        }
        match &proposal.kind {
            ProposalKind::BountyDone {
                bounty_id,
                receiver_id,
            } => {
                self.internal_execute_bounty_payout(*bounty_id, &receiver_id.clone().into(), false)
            }
            _ => PromiseOrValue::Value(()),
        }
    }

    pub(crate) fn internal_user_info(&self) -> UserInfo {
        let account_id = env::predecessor_account_id();
        UserInfo {
            amount: self.get_user_weight(&account_id),
            account_id,
        }
    }
}

#[near_bindgen]
impl Contract {
    /// Add proposal to this DAO.
    #[payable]
    pub fn add_proposal(&mut self, proposal: ProposalInput) -> u64 {
        // 0. validate bond attached.
        // TODO: consider bond in the token of this DAO.
        let policy = self.policy.get().unwrap().to_policy();

        assert_eq!(
            env::attached_deposit(),
            policy.proposal_bond.0,
            "ERR_MIN_BOND"
        );

        // 1. Validate proposal.
        match &proposal.kind {
            ProposalKind::ChangePolicy { policy } => match policy {
                VersionedPolicy::Current(_) => {}
                _ => panic!("ERR_INVALID_POLICY"),
            },
            ProposalKind::Transfer { token_id, msg, .. } => {
                assert!(
                    !(token_id == OLD_BASE_TOKEN) || msg.is_none(),
                    "ERR_BASE_TOKEN_NO_MSG"
                );
            }
            ProposalKind::SetStakingContract { .. } => assert!(
                self.staking_id.is_none(),
                "ERR_STAKING_CONTRACT_CANT_CHANGE"
            ),
            // TODO: add more verifications.
            _ => {}
        };

        // 2. Check permission of caller to add this type of proposal.
        assert!(
            policy
                .can_execute_action(
                    self.internal_user_info(),
                    &proposal.kind,
                    &Action::AddProposal
                )
                .1,
            "ERR_PERMISSION_DENIED"
        );

        // 3. Actually add proposal to the current list of proposals.
        let id = self.last_proposal_id;
        self.proposals
            .insert(&id, &VersionedProposal::Default(proposal.into()));
        self.last_proposal_id += 1;
        self.locked_amount += env::attached_deposit();
        id
    }

    /// Act on given proposal by id, if permissions allow.
    /// Memo is logged but not stored in the state. Can be used to leave notes or explain the action.
    pub fn act_proposal(&mut self, id: u64, action: Action, memo: Option<String>) {
        let mut proposal: Proposal = self.proposals.get(&id).expect("ERR_NO_PROPOSAL").into();
        let policy = self.policy.get().unwrap().to_policy();
        // Check permissions for the given action.
        let (roles, allowed) =
            policy.can_execute_action(self.internal_user_info(), &proposal.kind, &action);
        assert!(allowed, "ERR_PERMISSION_DENIED");
        let sender_id = env::predecessor_account_id();
        // Update proposal given action. Returns true if should be updated in storage.
        let update = match action {
            Action::AddProposal => env::panic_str("ERR_WRONG_ACTION"),
            Action::RemoveProposal => {
                self.proposals.remove(&id);
                false
            }
            Action::VoteApprove | Action::VoteReject | Action::VoteRemove => {
                assert!(
                    matches!(proposal.status, ProposalStatus::InProgress),
                    "ERR_PROPOSAL_NOT_READY_FOR_VOTE"
                );
                
                // **TODO** This is where we would do ValidateContractName
                // Here we would chech if the given user has the right to mint on that specific contract, or he/she only has the right to mint on another contract.
                // CheckRoleName("admin_nft.soundsplash.near", "nft.soundsplash.near");

                proposal.update_votes(
                    &sender_id,
                    &roles,
                    Vote::from(action),
                    &policy,
                    self.get_user_weight(&sender_id),
                );
                // Updates proposal status with new votes using the policy.
                proposal.status =
                    policy.proposal_status(&proposal, roles, self.total_delegation_amount);
                if proposal.status == ProposalStatus::Approved {
                    self.internal_execute_proposal(&policy, &proposal, id);
                    true
                } else if proposal.status == ProposalStatus::Removed {
                    self.internal_reject_proposal(&policy, &proposal, false);
                    self.proposals.remove(&id);
                    false
                } else if proposal.status == ProposalStatus::Rejected {
                    self.internal_reject_proposal(&policy, &proposal, true);
                    true
                } else {
                    // Still in progress or expired.
                    true
                }
            }
            // There are two cases when proposal must be finalized manually: expired or failed.
            // In case of failed, we just recompute the status and if it still approved, we re-execute the proposal.
            // In case of expired, we reject the proposal and return the bond.
            // Corner cases:
            //  - if proposal expired during the failed state - it will be marked as expired.
            //  - if the number of votes in the group has changed (new members has been added) -
            //      the proposal can loose it's approved state. In this case new proposal needs to be made, this one can only expire.
            Action::Finalize => {
                proposal.status = policy.proposal_status(
                    &proposal,
                    policy.roles.iter().map(|r| r.name.clone()).collect(),
                    self.total_delegation_amount,
                );
                match proposal.status {
                    ProposalStatus::Approved => {
                        self.internal_execute_proposal(&policy, &proposal, id);
                    }
                    ProposalStatus::Expired => {
                        self.internal_reject_proposal(&policy, &proposal, true);
                    }
                    _ => {
                        env::panic_str("ERR_PROPOSAL_NOT_EXPIRED_OR_FAILED");
                    }
                }
                true
            }
            Action::MoveToHub => false,
        };
        if update {
            self.proposals
                .insert(&id, &VersionedProposal::Default(proposal));
        }
        if let Some(memo) = memo {
            log!("Memo: {}", memo);
        }
    }

    /// Receiving callback after the proposal has been finalized.
    /// If successful, returns bond money to the proposal originator.
    /// If the proposal execution failed (funds didn't transfer or function call failure),
    /// move proposal to "Failed" state.
    #[private]
    pub fn on_proposal_callback(&mut self, proposal_id: u64) -> PromiseOrValue<()> {
        let mut proposal: Proposal = self
            .proposals
            .get(&proposal_id)
            .expect("ERR_NO_PROPOSAL")
            .into();
        assert_eq!(
            env::promise_results_count(),
            1,
            "ERR_UNEXPECTED_CALLBACK_PROMISES"
        );
        let result = match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => self.internal_callback_proposal_success(&mut proposal),
            PromiseResult::Failed => self.internal_callback_proposal_fail(&mut proposal),
        };
        self.proposals
            .insert(&proposal_id, &VersionedProposal::Default(proposal.into()));
        result
    }

    /// Test if caller (Artist) has the right to mint on the specific contract
    pub fn assert_artist_can_mint(&self, contract_name: AccountId) {
        let master_group = "master_".to_string() + &contract_name.to_string();  // Name of the MasterGroup
        let artist = UserInfo {                                                 // Artist as UserInfo struct
            account_id: env::predecessor_account_id(),
            amount: 0
        };
        // **TODO** should move policy.get() here

        for i in 0..self.policy.get().unwrap().to_policy().roles.len() {
            if &self.policy.get().unwrap().to_policy().roles[i].name == &master_group {
                log!("match_user(): {:?}", self.policy.get().unwrap().to_policy().roles[i].kind.match_user(&artist));
                assert!(
                    self.policy.get().unwrap().to_policy().roles[i].kind.match_user(&artist),
                    "You are not allowed to mint on this specific contract."
                );
                return;
            }
        }
        assert!(false, "The role was not found.");
    }
}

