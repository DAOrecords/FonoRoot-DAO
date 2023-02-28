use near_sdk::test_utils::{accounts, VMContextBuilder};
pub use near_sdk::json_types::{Base64VecU8, U128};
use std::collections::{HashMap};
use near_sdk::{testing_env, AccountId};
use crate::fonoroot_tests::helpers::{
    create_master_group_proposal, 
    add_member_to_master_proposal,
    prepare_nft_half_ready_proposal,
    update_nft_full_proposal,
    mint_root_proposal,
    add_revenue_table_proposal
};
use near_sdk_sim::to_yocto;
use crate::policy::{VersionedPolicy};
use crate::types::{Action, MintRootResult};
use crate::Contract;
use crate::Config;


/// Test if the values to the RevenueTable get inserted and the price is correct.
#[test]
fn create_revenue_table_correct_values_are_inserted() {
    let mut context = VMContextBuilder::new();
    testing_env!(context.predecessor_account_id(accounts(1)).build());                          // This is Bob
    let mut contract = Contract::new(
        Config::test_config(),
        VersionedPolicy::Default(vec![accounts(1).into()]),                                     // Council is Bob
    );

    // Create the master group
    let mut id = create_master_group_proposal(&mut context, &mut contract, "minting-contract-1.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);

    // Add Alice as member
    id = add_member_to_master_proposal(&mut context, &mut contract, accounts(0), "master_minting-contract-1.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);
    
    // Prepare NFT
    testing_env!(context.predecessor_account_id(accounts(0)).build());                          // Artist will be Alice
    testing_env!(context.signer_account_id(accounts(0)).build());
    id = prepare_nft_half_ready_proposal(&mut context, &mut contract);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Update the Prepared NFT
    id = update_nft_full_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Mint the prepared NFT
    id = mint_root_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    let example_result = MintRootResult {
        contract: AccountId::new_unchecked("minting-contract-1.near".to_string()),
        root_id: "fono-root-0".to_string(),
    };

    contract.mint_root_callback(Ok(example_result), accounts(0));

    let mut unchecked_table = HashMap::default();
    unchecked_table.insert(accounts(0), 9000);
    unchecked_table.insert(accounts(1), 1000);

    // Add a new RevenueTable
    id = add_revenue_table_proposal(&mut context, &mut contract, "fono-root-0".to_string(), AccountId::new_unchecked("minting-contract-1.near".to_string()), unchecked_table, U128(to_yocto("5")));
    contract.act_proposal(id, Action::VoteApprove, None);

    let catalogue_of_alice = contract.catalogues.get(&accounts(0)).unwrap();
    let inserted_revenue_table = catalogue_of_alice.get(&0).unwrap().unwrap();
    let income_table = contract.income_tables.get(&0).unwrap();

    assert_eq!(inserted_revenue_table.revenue_table.get(&accounts(0)), Some(&9000), "Alice should have 90% of the revenue of the newly minted song.");
    assert_eq!(inserted_revenue_table.revenue_table.get(&accounts(1)), Some(&1000), "Bob should have 10% of the revenue of the newly minted song.");

    assert_eq!(income_table.price, Some(U128(to_yocto("5"))), "The price of the new NFT should be 5 NEAR now.");
}

/// This should panic, because the caller is not the owner of the NFT
#[test]
#[should_panic(expected = "Only the owner (Artist) can alter the revenue table!")]
fn create_revenue_table_not_owner_error() {
    let mut context = VMContextBuilder::new();
    testing_env!(context.predecessor_account_id(accounts(1)).build());                          // This is Bob
    let mut contract = Contract::new(
        Config::test_config(),
        VersionedPolicy::Default(vec![accounts(1).into()]),                                     // Council is Bob
    );

    // Create the master group
    let mut id = create_master_group_proposal(&mut context, &mut contract, "minting-contract-1.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);

    // Add Alice and Bob as member
    id = add_member_to_master_proposal(&mut context, &mut contract, accounts(0), "master_minting-contract-1.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);
    id = add_member_to_master_proposal(&mut context, &mut contract, accounts(1), "master_minting-contract-1.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);
    
    // Prepare NFT
    testing_env!(context.predecessor_account_id(accounts(0)).build());                          // Artist will be Alice
    testing_env!(context.signer_account_id(accounts(0)).build());
    id = prepare_nft_half_ready_proposal(&mut context, &mut contract);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Update the Prepared NFT
    id = update_nft_full_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Mint the prepared NFT
    id = mint_root_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    let example_result = MintRootResult {
        contract: AccountId::new_unchecked("minting-contract-1.near".to_string()),
        root_id: "fono-root-0".to_string(),
    };

    contract.mint_root_callback(Ok(example_result), accounts(0));
    
    
    // Prepare NFT
    testing_env!(context.predecessor_account_id(accounts(1)).build());                          // Artist will be Bob
    testing_env!(context.signer_account_id(accounts(1)).build());
    id = prepare_nft_half_ready_proposal(&mut context, &mut contract);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Update the Prepared NFT
    id = update_nft_full_proposal(&mut context, &mut contract, 1);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Mint the prepared NFT
    id = mint_root_proposal(&mut context, &mut contract, 1);
    contract.act_proposal(id, Action::VoteApprove, None);

    let second_example_result = MintRootResult {
        contract: AccountId::new_unchecked("minting-contract-1.near".to_string()),
        root_id: "fono-root-1".to_string(),
    };

    contract.mint_root_callback(Ok(second_example_result), accounts(1));

    let mut unchecked_table = HashMap::default();
    unchecked_table.insert(accounts(0), 9000);
    unchecked_table.insert(accounts(1), 1000);

    // Add a new RevenueTable
    id = add_revenue_table_proposal(&mut context, &mut contract, "fono-root-0".to_string(), AccountId::new_unchecked("minting-contract-1.near".to_string()), unchecked_table, U128(to_yocto("5")));
    contract.act_proposal(id, Action::VoteApprove, None);
}

/// This should panic, because a RevenueTable alredy exists
#[test]
#[should_panic]
fn create_revenue_table_already_exists_error() {
    let mut context = VMContextBuilder::new();
    testing_env!(context.predecessor_account_id(accounts(1)).build());                          // This is Bob
    let mut contract = Contract::new(
        Config::test_config(),
        VersionedPolicy::Default(vec![accounts(1).into()]),                                     // Council is Bob
    );

    // Create the master group
    let mut id = create_master_group_proposal(&mut context, &mut contract, "minting-contract-1.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);

    // Add Alice as member
    id = add_member_to_master_proposal(&mut context, &mut contract, accounts(0), "master_minting-contract-1.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);
    
    // Prepare NFT
    testing_env!(context.predecessor_account_id(accounts(0)).build());                          // Artist will be Alice
    testing_env!(context.signer_account_id(accounts(0)).build());
    id = prepare_nft_half_ready_proposal(&mut context, &mut contract);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Update the Prepared NFT
    id = update_nft_full_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Mint the prepared NFT
    id = mint_root_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    let example_result = MintRootResult {
        contract: AccountId::new_unchecked("minting-contract-1.near".to_string()),
        root_id: "fono-root-0".to_string(),
    };

    contract.mint_root_callback(Ok(example_result), accounts(0));

    let mut unchecked_table = HashMap::default();
    unchecked_table.insert(accounts(0), 9000);
    unchecked_table.insert(accounts(1), 1000);

    // Add a new RevenueTable
    id = add_revenue_table_proposal(&mut context, &mut contract, "fono-root-0".to_string(), AccountId::new_unchecked("minting-contract-1.near".to_string()), unchecked_table, U128(to_yocto("5")));
    contract.act_proposal(id, Action::VoteApprove, None);

    // Try to add the RevenueTable again
    let mut new_unchecked_table = HashMap::default();
    new_unchecked_table.insert(accounts(2), 10000);
    id = add_revenue_table_proposal(&mut context, &mut contract, "fono-root-0".to_string(), AccountId::new_unchecked("minting-contract-1.near".to_string()), new_unchecked_table, U128(to_yocto("5")));
    contract.act_proposal(id, Action::VoteApprove, None);   
}

/// This should panic, because the sum of the entries does not add up to 100%
#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn create_revenue_table_table_not_correct() {
    let mut context = VMContextBuilder::new();
    testing_env!(context.predecessor_account_id(accounts(1)).build());                          // This is Bob
    let mut contract = Contract::new(
        Config::test_config(),
        VersionedPolicy::Default(vec![accounts(1).into()]),                                     // Council is Bob
    );

    // Create the master group
    let mut id = create_master_group_proposal(&mut context, &mut contract, "minting-contract-1.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);

    // Add Alice as member
    id = add_member_to_master_proposal(&mut context, &mut contract, accounts(0), "master_minting-contract-1.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);
    
    // Prepare NFT
    testing_env!(context.predecessor_account_id(accounts(0)).build());                          // Artist will be Alice
    testing_env!(context.signer_account_id(accounts(0)).build());
    id = prepare_nft_half_ready_proposal(&mut context, &mut contract);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Update the Prepared NFT
    id = update_nft_full_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Mint the prepared NFT
    id = mint_root_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    let example_result = MintRootResult {
        contract: AccountId::new_unchecked("minting-contract-1.near".to_string()),
        root_id: "fono-root-0".to_string(),
    };

    contract.mint_root_callback(Ok(example_result), accounts(0));

    let mut unchecked_table = HashMap::default();
    unchecked_table.insert(accounts(0), 9000);
    unchecked_table.insert(accounts(1), 1000);
    unchecked_table.insert(accounts(2), 1000);

    // Add a new RevenueTable
    id = add_revenue_table_proposal(&mut context, &mut contract, "fono-root-0".to_string(), AccountId::new_unchecked("minting-contract-1.near".to_string()), unchecked_table, U128(to_yocto("5")));
    contract.act_proposal(id, Action::VoteApprove, None);
}