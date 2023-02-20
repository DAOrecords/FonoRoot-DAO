use near_sdk::test_utils::{accounts, VMContextBuilder};
pub use near_sdk::json_types::{Base64VecU8, U128};
use std::collections::{HashSet};
use near_sdk::{log, testing_env, AccountId};
use crate::fonoroot_tests::helpers::{
    create_master_group_proposal, 
    add_member_to_master_proposal, 
    prepare_nft_half_ready_proposal
};
//use near_sdk_sim::to_yocto;
//use crate::proposals::{};
use crate::policy::{VersionedPolicy};
use crate::types::{Action};
use crate::Contract;
use crate::Config;


/// Prepare a half-ready InProgressMetadata, then update it
#[test]
fn update_half_ready_test() {
    let mut context = VMContextBuilder::new();
    testing_env!(context.predecessor_account_id(accounts(1)).build());                          // This is Bob
    let mut contract = Contract::new(
        Config::test_config(),
        VersionedPolicy::Default(vec![accounts(1).into()]),                                     // Council is Bob
    );
    assert_eq!(0, contract.in_progress_nonce, "The in_progress_nonce should be 0.");

    // Create the master group
    let mut id = create_master_group_proposal(&mut context, &mut contract, "minting-contract-1.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);

    // Add Alice as member
    id = add_member_to_master_proposal(&mut context, &mut contract, accounts(0), "master_minting-contract-1.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);
    
    // Prepare NFT
    testing_env!(context.predecessor_account_id(accounts(0)).build());                          // Artist will be Alice
    id = prepare_nft_half_ready_proposal(&mut context, &mut contract);
    contract.act_proposal(id, Action::VoteApprove, None);

    let the_inserted_data = contract.in_progress_nfts.get(&0).unwrap();

    // DO THE TEST HERE
    panic!("panic!");
}