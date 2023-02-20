use near_sdk::test_utils::{accounts, VMContextBuilder};
pub use near_sdk::json_types::{Base64VecU8, U128};
use std::collections::{HashSet};
use near_sdk::{testing_env};
use crate::fonoroot_tests::helpers::{create_master_group_proposal, add_member_to_master_proposal, remove_member_from_master_proposal};
use crate::policy::{VersionedPolicy, RoleKind};
use crate::types::{Action};
use crate::Contract;
use crate::Config;


#[test]
fn master_group_gets_created() {
    let mut context = VMContextBuilder::new();
    testing_env!(context.predecessor_account_id(accounts(1)).build());
    let mut contract = Contract::new(
        Config::test_config(),
        VersionedPolicy::Default(vec![accounts(1).into()]),
    );
    let id = create_master_group_proposal(&mut context, &mut contract, "master_minting-contract-1.near".to_string());
    assert_eq!(id,0, "Proposal ID should be 0");
    
    let mut policy = contract.policy.get().unwrap().to_policy();
    
    assert_eq!(1, policy.roles.len());
    contract.act_proposal(id, Action::VoteApprove, None);
    policy = contract.policy.get().unwrap().to_policy();

    assert_eq!(2, policy.roles.len(), "After adding Master Group, 2 roles should exist.");
}

#[test]
fn master_group_parameters_are_correct() {
    let mut context = VMContextBuilder::new();
    testing_env!(context.predecessor_account_id(accounts(1)).build());
    let mut contract = Contract::new(
        Config::test_config(),
        VersionedPolicy::Default(vec![accounts(1).into()]),
    );
    let id = create_master_group_proposal(&mut context, &mut contract, "minting-contract-1.near".to_string());
    
    contract.act_proposal(id, Action::VoteApprove, None);
    let policy = contract.policy.get().unwrap().to_policy();

    let expected_permissons: HashSet<String> = vec![
        "mint_root:*".to_string(),
        "prepair_nft:*".to_string(),
        "update_prepaired_nft:*".to_string(),
        "create_revenue_table:*".to_string(),
        "alter_revenue_table:*".to_string(),
        "payout_revenue:*".to_string()
    ].into_iter().collect();

    let zero_member: RoleKind = RoleKind::Group(vec![].into_iter().collect());

    assert_eq!("master_minting-contract-1.near", policy.roles[1].name, "Name of the master group should be master_minting-contract-1.near");
    assert_eq!(expected_permissons, policy.roles[1].permissions, "The permissons for the newly created master group are not correct.");
    assert_eq!(0, policy.roles[1].vote_policy.len(), "The vote policy for the master group should be empty (default).");
    assert_eq!(zero_member, policy.roles[1].kind, "The group should contain 0 members.");
}

/// This is the registration process
#[test]
fn master_group_member_tests() {
    let mut context = VMContextBuilder::new();
    testing_env!(context.predecessor_account_id(accounts(1)).build());
    let mut contract = Contract::new(
        Config::test_config(),
        VersionedPolicy::Default(vec![accounts(1).into()]),
    );
    let id = create_master_group_proposal(&mut context, &mut contract, "minting-contract-1.near".to_string());
    
    contract.act_proposal(id, Action::VoteApprove, None);
    let policy = contract.policy.get().unwrap().to_policy();

    let zero_member: RoleKind = RoleKind::Group(vec![].into_iter().collect());
    let one_member: RoleKind = RoleKind::Group(vec![accounts(0)].into_iter().collect());

    assert_eq!(zero_member, policy.roles[1].kind, "The group should contain 0 members.");
    let id = add_member_to_master_proposal(&mut context, &mut contract, accounts(0), "master_minting-contract-1.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);
    let policy = contract.policy.get().unwrap().to_policy();
    assert_eq!(one_member, policy.roles[1].kind, "The group should have Alice now as a member.");

    let id = remove_member_from_master_proposal(&mut context, &mut contract, accounts(0), "master_minting-contract-1.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);
    let policy = contract.policy.get().unwrap().to_policy();
    assert_eq!(zero_member, policy.roles[1].kind, "The group should contain 0 members after Alice was removed.");
}

/// This will remove all users from the group / unregister them
#[test]
fn master_group_reset_test() {
    let mut context = VMContextBuilder::new();
    testing_env!(context.predecessor_account_id(accounts(1)).build());
    let mut contract = Contract::new(
        Config::test_config(),
        VersionedPolicy::Default(vec![accounts(1).into()]),
    );
    let mut id = create_master_group_proposal(&mut context, &mut contract, "minting-contract-2.near".to_string());
    
    contract.act_proposal(id, Action::VoteApprove, None);
    id = add_member_to_master_proposal(&mut context, &mut contract, accounts(0), "master_minting-contract-2.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);
    id = add_member_to_master_proposal(&mut context, &mut contract, accounts(1), "master_minting-contract-2.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);
    id = add_member_to_master_proposal(&mut context, &mut contract, accounts(2), "master_minting-contract-2.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);
    let mut policy = contract.policy.get().unwrap().to_policy();
    assert_eq!(3, policy.roles[1].kind.get_role_size().unwrap(), "The Master Group should have 3 members");

    // This will reset the Master Group (0 member)
    id = create_master_group_proposal(&mut context, &mut contract, "minting-contract-2.near".to_string());
    contract.act_proposal(id, Action::VoteApprove, None);
    policy = contract.policy.get().unwrap().to_policy();
    assert_eq!(0, policy.roles[1].kind.get_role_size().unwrap(), "The Master Group should be empty (0 members");
}