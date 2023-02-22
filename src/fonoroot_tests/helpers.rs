use std::collections::{HashMap};
use near_sdk::test_utils::{VMContextBuilder};
use near_sdk::{log, AccountId};
pub use near_sdk::json_types::{Base64VecU8};
use near_sdk::testing_env;
use near_sdk_sim::to_yocto;
use crate::proposals::{ProposalInput, ProposalKind};
use crate::policy::{RoleKind, RolePermission};
use crate::types::{NftDataFromFrontEnd};
use crate::Contract;


/// Create a Master Group for the minting contract with @name
pub fn create_master_group_proposal(context: &mut VMContextBuilder, contract: &mut Contract, name: String) -> u64 {
    testing_env!(context.attached_deposit(to_yocto("0")).build());
    contract.add_proposal(ProposalInput {
        description: "Create Master Group".to_string(),
        kind: {
            ProposalKind::ChangePolicyAddOrUpdateRole {
                role: RolePermission {
                    name: format!("master_{}", name),
                    kind: RoleKind::Group(vec![].into_iter().collect()),
                    permissions: vec![
                            "mint_root:*".to_string(),
                            "prepair_nft:*".to_string(),
                            "update_prepaired_nft:*".to_string(),
                            "create_revenue_table:*".to_string(),
                            "alter_revenue_table:*".to_string(),
                            "payout_revenue:*".to_string()
                        ].into_iter().collect(),
                    vote_policy: HashMap::default(),
                }
            }
        }
    })
}

/// Add member to master group
pub fn add_member_to_master_proposal(context: &mut VMContextBuilder, contract: &mut Contract, artist: AccountId, master: String) -> u64 {
    testing_env!(context.attached_deposit(to_yocto("0")).build());
    contract.add_proposal(ProposalInput {
        description: "Add member to Master Group".to_string(),
        kind: {
            ProposalKind::AddMemberToRole { 
                member_id: artist, 
                role: master 
            }
        }
    })
}

/// Remove member from master group
pub fn remove_member_from_master_proposal(context: &mut VMContextBuilder, contract: &mut Contract, artist: AccountId, master: String) -> u64 {
    testing_env!(context.attached_deposit(to_yocto("0")).build());
    contract.add_proposal(ProposalInput {
        description: "Remove member from Master Group".to_string(),
        kind: {
            ProposalKind::RemoveMemberFromRole {
                member_id: artist,
                role: master
            }
        }
    })
}

/// This will add an InProgressNft to the list that is ready to be minted.
pub fn prepare_nft_full_proposal(context: &mut VMContextBuilder, contract: &mut Contract) -> u64 {
    testing_env!(context.attached_deposit(to_yocto("0")).build());
    contract.add_proposal(ProposalInput {
        description: "Prepare NFT".to_string(),
        kind: ProposalKind::PrepareNft {
            nft_data: NftDataFromFrontEnd {
                contract: AccountId::new_unchecked("minting-contract-1.near".to_string()),
                title: Some("Test NFT".to_string()),
                desc: Some("Description about a Test NFT".to_string()),
                image_cid: Some("QmerincKVRPTXh1z41725mFNvGp31UBgfyms5xWi1taNuQ".to_string()),
                image_hash: Some(Base64VecU8(vec![1,2,3])),
                music_folder_cid: Some("QmU51uX3B44Z4pH2XimaJ6eScRgAzG4XUrKfsz1yWVCo6f".to_string()),
                music_folder_hash: Some(Base64VecU8(vec![4,5,6])),
                animation_url: Some("https://ipfs.io/ipfs/QmU51uX3B44Z4pH2XimaJ6eScRgAzG4XUrKfsz1yWVCo6f".to_string()),
                animation_url_hash: Some(Base64VecU8(vec![7,8,9])),
                meta_json_cid: Some("QmeCBSWQcDwn3ktKdEfDn68bt2PQbx3khyqGQAeYAVUHpb".to_string()),
                meta_json_hash: Some(Base64VecU8(vec![10,11,12])),
            }
        },
    })
}

/// This will try to add a bad InProgressNft, where there is image, but no hash for image.
pub fn prepare_nft_image_hash_missing_proposal(context: &mut VMContextBuilder, contract: &mut Contract) -> u64 {
    testing_env!(context.attached_deposit(to_yocto("0")).build());
    contract.add_proposal(ProposalInput {
        description: "Prepare NFT".to_string(),
        kind: ProposalKind::PrepareNft {
            nft_data: NftDataFromFrontEnd {
                contract: AccountId::new_unchecked("minting-contract-1.near".to_string()),
                title: Some("Test NFT".to_string()),
                desc: Some("Description about a Test NFT".to_string()),
                image_cid: Some("QmerincKVRPTXh1z41725mFNvGp31UBgfyms5xWi1taNuQ".to_string()),
                image_hash: None,
                music_folder_cid: Some("QmU51uX3B44Z4pH2XimaJ6eScRgAzG4XUrKfsz1yWVCo6f".to_string()),
                music_folder_hash: Some(Base64VecU8(vec![4,5,6])),
                animation_url: Some("https://ipfs.io/ipfs/QmU51uX3B44Z4pH2XimaJ6eScRgAzG4XUrKfsz1yWVCo6f".to_string()),
                animation_url_hash: Some(Base64VecU8(vec![7,8,9])),
                meta_json_cid: Some("QmeCBSWQcDwn3ktKdEfDn68bt2PQbx3khyqGQAeYAVUHpb".to_string()),
                meta_json_hash: Some(Base64VecU8(vec![10,11,12])),
            }
        },
    })
}

/// This will try to add a bad InProgressNft, where there is meta, but no hash for meta.
pub fn prepare_nft_meta_hash_missing_proposal(context: &mut VMContextBuilder, contract: &mut Contract) -> u64 {
    testing_env!(context.attached_deposit(to_yocto("0")).build());
    contract.add_proposal(ProposalInput {
        description: "Prepare NFT".to_string(),
        kind: ProposalKind::PrepareNft {
            nft_data: NftDataFromFrontEnd {
                contract: AccountId::new_unchecked("minting-contract-1.near".to_string()),
                title: Some("Test NFT".to_string()),
                desc: Some("Description about a Test NFT".to_string()),
                image_cid: Some("QmerincKVRPTXh1z41725mFNvGp31UBgfyms5xWi1taNuQ".to_string()),
                image_hash: Some(Base64VecU8(vec![1,2,3])),
                music_folder_cid: Some("QmU51uX3B44Z4pH2XimaJ6eScRgAzG4XUrKfsz1yWVCo6f".to_string()),
                music_folder_hash: Some(Base64VecU8(vec![4,5,6])),
                animation_url: Some("https://ipfs.io/ipfs/QmU51uX3B44Z4pH2XimaJ6eScRgAzG4XUrKfsz1yWVCo6f".to_string()),
                animation_url_hash: Some(Base64VecU8(vec![7,8,9])),
                meta_json_cid: Some("QmeCBSWQcDwn3ktKdEfDn68bt2PQbx3khyqGQAeYAVUHpb".to_string()),
                meta_json_hash: None,
            }
        },
    })
}

/// This will add an InProgressNft to the list that is not ready to be minted, it is half-ready
pub fn prepare_nft_half_ready_proposal(context: &mut VMContextBuilder, contract: &mut Contract) -> u64 {
    testing_env!(context.attached_deposit(to_yocto("0")).build());
    contract.add_proposal(ProposalInput {
        description: "Prepare NFT".to_string(),
        kind: ProposalKind::PrepareNft {
            nft_data: NftDataFromFrontEnd {
                contract: AccountId::new_unchecked("minting-contract-1.near".to_string()),
                title: Some("Test NFT".to_string()),
                desc: None,
                image_cid: Some("QmerincKVRPTXh1z41725mFNvGp31UBgfyms5xWi1taNuQ".to_string()),
                image_hash: Some(Base64VecU8(vec![1,2,3])),
                music_folder_cid: Some("QmU51uX3B44Z4pH2XimaJ6eScRgAzG4XUrKfsz1yWVCo6f".to_string()),
                music_folder_hash: Some(Base64VecU8(vec![4,5,6])),
                animation_url: Some("https://ipfs.io/ipfs/QmU51uX3B44Z4pH2XimaJ6eScRgAzG4XUrKfsz1yWVCo6f".to_string()),
                animation_url_hash: Some(Base64VecU8(vec![7,8,9])),
                meta_json_cid: None,
                meta_json_hash: None,
            }
        },
    })
}

/// This will update an InProgressNft with the below data. After this, the NFT will be ready to be minted.
pub fn update_nft_full_proposal(context: &mut VMContextBuilder, contract: &mut Contract, id: u64) -> u64 {
    testing_env!(context.attached_deposit(to_yocto("0")).build());
    contract.add_proposal(ProposalInput {
        description: "Update prepared NFT (will be ready after this)".to_string(),
        kind: ProposalKind::UpdatePrepairedNft {
            id: id,
            new_nft_data: NftDataFromFrontEnd {
                contract: AccountId::new_unchecked("minting-contract-1.near".to_string()),
                title: Some("Test NFT".to_string()),
                desc: Some("Description of the updated NFT".to_string()),
                image_cid: Some("QmerincKVRPTXh1z41725mFNvGp31UBgfyms5xWi1taNuQ".to_string()),
                image_hash: Some(Base64VecU8(vec![1,2,3])),
                music_folder_cid: Some("QmU51uX3B44Z4pH2XimaJ6eScRgAzG4XUrKfsz1yWVCo6f".to_string()),
                music_folder_hash: Some(Base64VecU8(vec![4,5,6])),
                animation_url: Some("https://ipfs.io/ipfs/QmU51uX3B44Z4pH2XimaJ6eScRgAzG4XUrKfsz1yWVCo6f".to_string()),
                animation_url_hash: Some(Base64VecU8(vec![7,8,9])),
                meta_json_cid: Some("QmxCBSWQcDwn3ktKdEfDn68bt2PQbx3khyqGQAeYAVabcd".to_string()),
                meta_json_hash: Some(Base64VecU8(vec![15,16,17])),
            }
        },
    })
}

/// This will try to update an InProgressNft, but the data is not good, music_folder_hash is missing.
pub fn update_nft_music_hash_missing_proposal(context: &mut VMContextBuilder, contract: &mut Contract, id: u64) -> u64 {
    testing_env!(context.attached_deposit(to_yocto("0")).build());
    contract.add_proposal(ProposalInput {
        description: "Update prepared NFT (will be ready after this)".to_string(),
        kind: ProposalKind::UpdatePrepairedNft {
            id: id,
            new_nft_data: NftDataFromFrontEnd {
                contract: AccountId::new_unchecked("minting-contract-1.near".to_string()),
                title: Some("Test NFT".to_string()),
                desc: Some("Description of the updated NFT".to_string()),
                image_cid: Some("QmerincKVRPTXh1z41725mFNvGp31UBgfyms5xWi1taNuQ".to_string()),
                image_hash: Some(Base64VecU8(vec![1,2,3])),
                music_folder_cid: Some("QmU51uX3B44Z4pH2XimaJ6eScRgAzG4XUrKfsz1yWVCo6f".to_string()),
                music_folder_hash: None,
                animation_url: Some("https://ipfs.io/ipfs/QmU51uX3B44Z4pH2XimaJ6eScRgAzG4XUrKfsz1yWVCo6f".to_string()),
                animation_url_hash: Some(Base64VecU8(vec![7,8,9])),
                meta_json_cid: Some("QmxCBSWQcDwn3ktKdEfDn68bt2PQbx3khyqGQAeYAVabcd".to_string()),
                meta_json_hash: Some(Base64VecU8(vec![15,16,17])),
            }
        },
    })
}

/// This will update an InProgressNft with the below data. After this, the NFT still won't be ready to be minted, some data is missing.
pub fn update_nft_half_ready_proposal(context: &mut VMContextBuilder, contract: &mut Contract, id: u64) -> u64 {
    testing_env!(context.attached_deposit(to_yocto("0")).build());
    contract.add_proposal(ProposalInput {
        description: "Update prepared NFT (will be ready after this)".to_string(),
        kind: ProposalKind::UpdatePrepairedNft {
            id: id,
            new_nft_data: NftDataFromFrontEnd {
                contract: AccountId::new_unchecked("minting-contract-1.near".to_string()),
                title: Some("Test NFT".to_string()),
                desc: Some("Description of the updated NFT".to_string()),
                image_cid: Some("QmerincKVRPTXh1z41725mFNvGp31UBgfyms5xWi1taNuQ".to_string()),
                image_hash: Some(Base64VecU8(vec![1,2,3])),
                music_folder_cid: Some("QmU51uX3B44Z4pH2XimaJ6eScRgAzG4XUrKfsz1yWVCo6f".to_string()),
                music_folder_hash: Some(Base64VecU8(vec![4,5,6])),
                animation_url: Some("https://ipfs.io/ipfs/QmU51uX3B44Z4pH2XimaJ6eScRgAzG4XUrKfsz1yWVCo6f".to_string()),
                animation_url_hash: Some(Base64VecU8(vec![7,8,9])),
                meta_json_cid: None,
                meta_json_hash: None,
            }
        },
    })
}

/// This will mint the NFT that was previously prepared by prepare_nft_add_proposal
pub fn mint_root_add_proposal(context: &mut VMContextBuilder, contract: &mut Contract) -> u64 {
    log!("hello world.");
    testing_env!(context.attached_deposit(to_yocto("0")).build());
    contract.add_proposal(ProposalInput {
        description: "Mint Root NFT".to_string(),
        kind: ProposalKind::MintRoot {
            id: 0
        },
    })
}