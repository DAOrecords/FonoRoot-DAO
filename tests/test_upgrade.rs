use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::Base58CryptoHash;
use near_sdk::serde_json::json;
use near_sdk::AccountId;

use near_sdk_sim::{call, init_simulator, to_yocto, DEFAULT_GAS};
use sputnikdao2::{Action, Config, ProposalInput, ProposalKind, VersionedPolicy};

mod utils;
use crate::utils::*;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    DAO_WASM_BYTES => "res/sputnikdao2.wasm",
    OTHER_WASM_BYTES => "res/ref_exchange_release.wasm"
}


#[test]
fn our_other_own_test() {
    assert_ne!(
        2,
        3,
        "2 is equal 3. That's a problem."
    );
}