use near_sdk::test_utils::{accounts, VMContextBuilder};
pub use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::{log, testing_env, AccountId};
use crate::fonoroot_tests::helpers::{
    create_master_group_proposal, 
    add_member_to_master_proposal,
    remove_member_from_master_proposal,
    prepare_nft_half_ready_proposal,
    update_nft_full_proposal,
    mint_root_proposal
};
use crate::policy::{VersionedPolicy};
use crate::types::{Action, MintRootResult, UniqId};
use crate::Contract;
use crate::Config;


/// Possible to mint NFT, InProgressNft will be removed
#[test]
fn mint_root_removes_prepared_nft() {
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
    id = prepare_nft_half_ready_proposal(&mut context, &mut contract);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Update the Prepared NFT
    id = update_nft_full_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Mint the prepared NFT
    assert!(contract.in_progress_nfts.get(&0).is_some(), "There should be an entry here");
    id = mint_root_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    assert!(contract.in_progress_nfts.get(&0).is_none(), "The entry should have been removed at this point.");
}

/// This should panic, because the caller is not owner, altough he is also member of the master group
#[test]
#[should_panic(expected = "Only the owner of the draft can mint!")]
fn mint_root_not_owner_error() {
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
    id = prepare_nft_half_ready_proposal(&mut context, &mut contract);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Update the Prepared NFT
    id = update_nft_full_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Mint the prepared NFT
    testing_env!(context.predecessor_account_id(accounts(1)).build());                          // Bob is trying to mint an NFT that does not belong to him.
    id = mint_root_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);
}

/// This should fail, because the caller is not member of the master group any more
#[test]
#[should_panic(expected = "ERR_PERMISSION_DENIED")]
fn mint_root_not_member_error() {
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
    id = prepare_nft_half_ready_proposal(&mut context, &mut contract);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Update the Prepared NFT
    id = update_nft_full_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Remove Alice from the master group
    id = remove_member_from_master_proposal(&mut context, &mut contract, accounts(0), "master_minting-contract-1.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);

    // Mint the prepared NFT
    id = mint_root_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);
}

/// This should panic, because the promise result is Error
#[test]
#[should_panic(expected = "MintRoot promise come back with an error.")]
fn mint_root_callback_promise_returned_error() {
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
    id = prepare_nft_half_ready_proposal(&mut context, &mut contract);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Update the Prepared NFT
    id = update_nft_full_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Mint the prepared NFT
    id = mint_root_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    contract.mint_root_callback(Err(near_sdk::PromiseError::Failed), accounts(0));
}

/// Test if the TreeIndex is correct, and that the expected UniqId was generated
#[test]
fn mint_root_callback_tree_index() {
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

    let uniq_id = UniqId::new(AccountId::new_unchecked("minting-contract-1.near".to_string()), "fono-root-0".to_string());
    assert_eq!(0, contract.uniq_id_to_tree_index.get(&uniq_id).unwrap(), "The TreeIndex for the newly inserted NFT should be 0");
    assert_eq!(1, contract.tree_index, "TreeIndex should be 1 at this point.");
}


/// This should panic, "Duplicate TreeIndex error"
#[test]
#[should_panic(expected = "Duplicate TreeIndex error!")]
fn mint_root_callback_duplicate_tree_index_error() {
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
    id = prepare_nft_half_ready_proposal(&mut context, &mut contract);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Update the Prepared NFT
    id = update_nft_full_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Mint the prepared NFT
    id = mint_root_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    let first_result = MintRootResult {
        contract: AccountId::new_unchecked("minting-contract-1.near".to_string()),
        root_id: "fono-root-0".to_string(),
    };

    let second_result = MintRootResult {
        contract: AccountId::new_unchecked("minting-contract-1.near".to_string()),
        root_id: "fono-root-1".to_string(),
    };

    contract.mint_root_callback(Ok(first_result), accounts(0));
    contract.tree_index = 0;
    contract.mint_root_callback(Ok(second_result), accounts(0));
}

/// This should fail, because the UniqId already exists
#[test]
#[should_panic(expected = "The UniqId already exists!")]
fn mint_root_callback_uniq_id_error() {
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
    id = prepare_nft_half_ready_proposal(&mut context, &mut contract);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Update the Prepared NFT
    id = update_nft_full_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Mint the prepared NFT
    id = mint_root_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    let result = MintRootResult {
        contract: AccountId::new_unchecked("minting-contract-1.near".to_string()),
        root_id: "fono-root-0".to_string(),
    };

    contract.mint_root_callback(Ok(result.clone()), accounts(0));
    contract.mint_root_callback(Ok(result), accounts(0));
}

/// IncomeTable test
#[test]
fn mint_root_callback_income_table_test() {
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
    let inserted_income_table = contract.income_tables.get(&0).unwrap();
    
    assert_eq!(0, inserted_income_table.total_income, "The total_income should be 0");
    assert_eq!(0, inserted_income_table.current_balance, "The current_balance should be 0");
    assert_eq!("fono-root-0", inserted_income_table.root_id, "root_id should be fono-root-0");
    assert_eq!(AccountId::new_unchecked("minting-contract-1.near".to_string()), inserted_income_table.contract, "minting-contract-1.near");
    assert_eq!(accounts(0), inserted_income_table.owner, "Owner should be Alice");
    assert!(inserted_income_table.price.is_none(), "Price should be None");
}

/// Catalogue tests
#[test]
fn mint_root_callback_catalogue_test() {
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
    id = prepare_nft_half_ready_proposal(&mut context, &mut contract);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Update the Prepared NFT
    id = update_nft_full_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Mint the prepared NFT
    id = mint_root_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    let first_result = MintRootResult {
        contract: AccountId::new_unchecked("minting-contract-1.near".to_string()),
        root_id: "fono-root-0".to_string(),
    };

    contract.mint_root_callback(Ok(first_result), accounts(0));
    let alice_catalogue = contract.catalogues.get(&accounts(0)).unwrap();                       // Get the Catalogue for Alice

    assert_eq!(1, alice_catalogue.len(), "The Catalogue for Alice should have exactly 1 entry");
    assert!(alice_catalogue.get(&0).unwrap().is_none(), "There should be an entry on index 0, the value of it should be None.");
    assert!(alice_catalogue.get(&1).is_none(), "On index 1, there shouldn't be an entry.");
    
    // Prepare NFT
    testing_env!(context.predecessor_account_id(accounts(0)).build());                          // Artist will be Alice
    id = prepare_nft_half_ready_proposal(&mut context, &mut contract);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Update the Prepared NFT
    id = update_nft_full_proposal(&mut context, &mut contract, 1);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Mint the prepared NFT
    id = mint_root_proposal(&mut context, &mut contract, 1);
    contract.act_proposal(id, Action::VoteApprove, None);

    let second_result = MintRootResult {
        contract: AccountId::new_unchecked("minting-contract-1.near".to_string()),
        root_id: "fono-root-1".to_string(),
    };

    contract.mint_root_callback(Ok(second_result), accounts(0));
    let alice_catalogue = contract.catalogues.get(&accounts(0)).unwrap();                       // Get the Catalogue for Alice

    assert_eq!(2, alice_catalogue.len(), "The Catalogue for Alice should have exactly 2 entry");
    assert!(alice_catalogue.get(&0).unwrap().is_none(), "There should be an entry on index 0, the value of it should be None.");
    assert!(alice_catalogue.get(&1).unwrap().is_none(), "There should be an entry on index 1, the value of it should be None.");
}

/// This should panic, because the InProgressNft is not ready
#[test]
#[should_panic]
fn mint_root_not_ready_error() {
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
    id = prepare_nft_half_ready_proposal(&mut context, &mut contract);
    contract.act_proposal(id, Action::VoteApprove, None);

    // Mint the prepared NFT
    id = mint_root_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);
}