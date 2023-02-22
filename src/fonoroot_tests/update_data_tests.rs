use near_sdk::test_utils::{accounts, VMContextBuilder};
pub use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::{testing_env, AccountId};
use crate::fonoroot_tests::helpers::{
    create_master_group_proposal, 
    add_member_to_master_proposal,
    remove_member_from_master_proposal,
    prepare_nft_half_ready_proposal,
    update_nft_full_proposal,
    update_nft_music_hash_missing_proposal,
};
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

    // Update the Prepared NFT
    id = update_nft_full_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);

    let the_inserted_data = contract.in_progress_nfts.get(&0).unwrap();

    // This is all arbitrary data from front end
    assert_eq!("Test NFT".to_string(), the_inserted_data.title.unwrap(), "The Title is not correct!");
    assert_eq!("Description of the updated NFT".to_string(), the_inserted_data.desc.unwrap(), "The Description is not correct!");
    assert_eq!(AccountId::new_unchecked("minting-contract-1.near".to_string()), the_inserted_data.contract, "The Minting Contract is not correctly set!");
    assert_eq!("QmerincKVRPTXh1z41725mFNvGp31UBgfyms5xWi1taNuQ".to_string(), the_inserted_data.image.unwrap(), "The Image CID is not correct!");
    assert_eq!(Base64VecU8(vec![1,2,3]), the_inserted_data.image_hash.unwrap(), "The Image hash is not correct!");
    assert_eq!("QmU51uX3B44Z4pH2XimaJ6eScRgAzG4XUrKfsz1yWVCo6f".to_string(), the_inserted_data.music.unwrap(), "The Music Folder CID is not correct!");
    assert_eq!(Base64VecU8(vec![4,5,6]), the_inserted_data.music_hash.unwrap(), "The Music Folder hash is not correct!");
    assert_eq!("https://ipfs.io/ipfs/QmU51uX3B44Z4pH2XimaJ6eScRgAzG4XUrKfsz1yWVCo6f".to_string(), the_inserted_data.animation_url.unwrap(), "The Animation URL is not correct!");
    assert_eq!(Base64VecU8(vec![7,8,9]), the_inserted_data.animation_url_hash.unwrap(), "The Animation URL hash is not correct!");
    assert_eq!("QmxCBSWQcDwn3ktKdEfDn68bt2PQbx3khyqGQAeYAVabcd".to_string(), the_inserted_data.meta.unwrap(), "The Meta JSON CID is not correct!");
    assert_eq!(Base64VecU8(vec![15,16,17]), the_inserted_data.meta_hash.unwrap(), "The Meta JSON hash is not correct!");

    assert_eq!(0, the_inserted_data.id, "The ID of the newly inserted data should be 0.");
    assert_eq!(accounts(0), the_inserted_data.artist, "Artist (owner) should be Alice.");
    assert_eq!(1, contract.in_progress_nonce, "The in_progress_nonce should be 1 now.");
}

/// This should fail, because the frond end is trying to add a music CID to the InProgressMetadata, without a hash
#[test]
#[should_panic(expected = "Hash has to exist, if music folder exists!")]
fn update_music_hash_error() {
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

    // Update the Prepared NFT
    id = update_nft_music_hash_missing_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);
}

/// This should fail, because Alice is not member of the master group anymore
#[test]
#[should_panic(expected = "ERR_PERMISSION_DENIED")]
fn update_prepared_not_member_error() {
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

    // Remove Alice from the master group
    id = remove_member_from_master_proposal(&mut context, &mut contract, accounts(0), "master_minting-contract-1.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);

    // Update the Prepared NFT
    id = update_nft_music_hash_missing_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);
}

/// This should panic, because there is no data on given ID
#[test]
#[should_panic]
fn update_prepared_wrong_id() {
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

    // Update the Prepared NFT
    id = update_nft_full_proposal(&mut context, &mut contract, 1);
    contract.act_proposal(id, Action::VoteApprove, None);
}

/// This should fail, because Bob is not owner, even though he is member of the master group
#[test]
#[should_panic]
fn update_prepared_not_owner_error() {
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
    testing_env!(context.predecessor_account_id(accounts(1)).build());                          // Bob is trying to update the data for an NFT that he didn't start to prepare
    id = update_nft_full_proposal(&mut context, &mut contract, 0);
    contract.act_proposal(id, Action::VoteApprove, None);
}