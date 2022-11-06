use std::collections::HashMap;

use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::{env, AccountId};
use near_sdk_sim::{call, init_simulator, to_yocto, view};

use crate::utils::*;
use sputnikdao2::{
    default_policy, Action, BountyClaim, BountyOutput, Config, Policy, Proposal, ProposalInput,
    ProposalKind, ProposalOutput, ProposalStatus, RoleKind, RolePermission, VersionedPolicy,
    VotePolicy,
};

mod utils;

fn user(id: u32) -> AccountId {
    format!("user{}", id).parse().unwrap()
}


#[test]
fn our_own_test() {
    assert_eq!(
        2,
        2,
        "2 is not equal 2. That's a problem."
    );
}