import { connect, Contract, keyStores, WalletConnection, utils, KeyPair } from 'near-api-js';
import * as nearAPI from "near-api-js";
const CryptoJS = require('crypto-js');

const mode = 'development';       // 'mainnet' || 'development'

/** Real config. It's async. It was important when we tried to clone the site, so the config is not burnt in */
async function getRealConfig(env) {
  let contractName;
  try {
    contractName = await getContractName();
  } catch (error) {
    console.error(error) 
  }
  const { keyStores } = nearAPI;
  const keyStore = new keyStores.BrowserLocalStorageKeyStore();

  switch (env) {
    case 'development':
      return {
        networkId: 'testnet',
        nodeUrl: 'https://rpc.testnet.near.org',
        keyStore,
        contractName: contractName,
        walletUrl: 'https://wallet.testnet.near.org',
        helperUrl: 'https://helper.testnet.near.org',
        explorerUrl: 'https://explorer.testnet.near.org',
      }
    case 'mainnet':
      return {
        networkId: 'mainnet',
        nodeUrl: 'https://rpc.mainnet.near.org',
        contractName: contractName,
        walletUrl: 'https://wallet.near.org',
        helperUrl: 'https://helper.mainnet.near.org',
        explorerUrl: 'https://explorer.mainnet.near.org',
      }
    default:
      throw Error(`env is needed`);
  }
}

export async function getContractName() {
  const fetchObj = await fetch(window.location.origin + window.location.pathname + '/projectConfig.json')
  .then((response) => response.json())
  .catch((err) => console.error(err));
  return fetchObj.contractName;
}


// Initialize contract & set global variables
export async function initContract() {
  const nearConfig = await getRealConfig(mode);
  const near = await connect(Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } }, nearConfig));
  
  window.walletConnection = new WalletConnection(near)  
  window.accountId = window.walletConnection.getAccountId()                                // Getting the Account ID. If still unauthorized, it's just empty string
  
  window.contract = await new Contract(window.walletConnection.account(), nearConfig.contractName, {
    viewMethods: ["get_policy", "get_last_proposal_id", "get_proposals", "get_in_progress_nfts"],
    changeMethods: ["add_proposal", "act_proposal"],
  })
}

// Create (master) group
export async function createGroup(group) {
  let success = false;
  const args = {
    proposal: {
      description: "Create Master Group",
      kind: {
        ChangePolicyAddOrUpdateRole: {
          role: {
            name: group,
            kind: {
              Group: []
            },
            permissions: [
              "MintRoot:*",
              "PrepairNft:*",
              "UpdatePrepairedNft:*",
              "ScheduleMint:*"
            ],
            vote_policy: {
              // Should be 1 / Infinity
            }
          }
        }
      }
    }
  }

  const gas = 100000000000000;
  const amount = utils.format.parseNearAmount("1");

  await window.contract.add_proposal(args, gas, amount)
    .then((msg) => { 
      console.log("Success! (Create new Master Group)", msg); 
      success = true;
    })
    .catch((err) => console.error(`There was an error while trying to create new Master Group ${group}: `, err));
  
  return success;
}

// Prepair Data (create new entry in `in_progress_nfts`)
export async function prepairNft(newNftDetails) {
  let inProgressID = -1;

  const args = {
    proposal: {
      description: `Prepair NFT: ${newNftDetails.title}`,
      kind: {
        PrepairNft: {
          nft_data: {
            contract: newNftDetails.contract,
            title: newNftDetails.title,
            desc: newNftDetails.description,
            image_cid: newNftDetails.image_cid,
            music_folder_cid: newNftDetails.music_cid,
            meta_json_cid: newNftDetails.meta_cid,
          }
        }
      }
    }
  };

  const gas = 100000000000000;
  const amount = utils.format.parseNearAmount("1");
  
  await window.contract.add_proposal(args, gas, amount)
    .then((id) => {
      console.log("Success! (Prepair NFT)", msg);
      // not good, will not get ID this way (otherwise working)
      inProgressID = id;
    })
    .catch((err) => console.error(`There was an error while prepairing NFT data. NFT title: ${newNftDetails.title}`, err));

  return inProgressID
}

// Update Data (overwrite an entry in `in_progress_nfts`)
export async function updateNft(id, updatedNftDetails) {
  let success = false;

  const args = {
    proposal: {
      description: `Update NFT: ${updatedNftDetails.title}`,
      kind: {
        UpdatePrepairedNft: {
          id: id,
          new_nft_data: {
            contract: updatedNftDetails.contract,
            title: updatedNftDetails.title,
            desc: updatedNftDetails.description,
            image_cid: updatedNftDetails.image_cid,
            music_folder_cid: updatedNftDetails.music_cid,
            meta_json_cid: updatedNftDetails.meta_cid,
          }
        }
      }
    }
  };
console.log(args)
  const gas = 100000000000000;
  const amount = utils.format.parseNearAmount("1");

  await window.contract.add_proposal(args, gas, amount)
    .then((msg) => {
      console.log("Success! (Updating NFT)", msg);
      success = true;
    })
    .catch((err) => console.error(`There was an error while updating NFT data. ID: ${id}`, err));

  return success;
}

// Mint the NFT (user initiated)
export async function mintNft(id) {
  let success = false;

  const args = {
    proposal: {
      description: `MintRoot. ID: : ${id}`,
      kind: {
        MintRoot: {
          id: id
        }
      }
    }
  };

console.table(args);

  const gas = 100000000000000;
  const amount = utils.format.parseNearAmount("1");

  await window.contract.add_proposal(args, gas, amount)
    .then((msg) => {
      console.log("Success! (MintRoot)", msg);
      success = true;
    })
    .catch((err) => console.error(`There was an error while minting NFT. ID: ${id}`, err));

  return success;
}

// Add Artist to master group of the specific contract
export async function registerUser(user, group) {
  let success = false;
  const args = {
    proposal: {
      description: "Add Artist to master group (or collab to collab group)",
      kind: {
        AddMemberToRole: {
          member_id: user,
          role: group,
        }
      }
    }
  }

  const gas = 100000000000000;
  const amount = utils.format.parseNearAmount("1");

  await window.contract.add_proposal(args, gas, amount)
    .then((msg) => { 
      console.log("Success! (Add user to group)", msg); 
      success = true; 
    })
    .catch((err) => console.error(`There was an error while trying to add ${user} to ${group}: `, err));
  
  return success;
}

// Act on Proposal
export async function actOnProposal(proposalId) {
  let message = "not_succeed";

  const args = {
    id: proposalId,
    action: "VoteApprove"
  }

  const gas = 300000000000000;

  await window.contract.act_proposal(args, gas)
    .then((msg) => {
      console.log("Success! (Act on proposal), ", msg);
      message = "Success! (Act on proposal), " +  msg;
    })
    .catch((err) => {
      console.error(`There was an error while trying to act on proposal with proposal ID ${proposalId}`, err);
      message = `There was an error while trying to act on proposal with proposal ID ${proposalId} ` +  err
    });

    return message;
}

// Get last proposal ID
export async function getLastProposalId() {
  let lastProposalId = -1;

  await window.contract.get_last_proposal_id()
    .then((response) => {
      console.log("Got last proposal ID: ", response);
      lastProposalId = response;
    })
    .catch((err) => console.error("There was an error while trying to get the last proposal ID: ", err));

    return lastProposalId;
}

// Get list of proposals from index
export async function getListOfProposals(index) {
  let proposalList = [];

  const args = {
    "from_index": (index>=0) ? index : 0,
    "limit": 10
  }

  await window.contract.get_proposals(args)
    .then((response) => {
      console.log("Successfully got the list of proposals from index ", index);
      proposalList = response;
    })
    .catch((err) => console.error(`There was an error while trying to get the list of proposals from index ${index}, `, err));

    return proposalList;
}

// Get list of all InProgress NFTs
export async function getListOfAllInProgressNfts() {
  let inProgressNfts = [];

  await window.contract.get_in_progress_nfts()
    .then((list) => {
      console.log("Successfully got the list of all InProgress NFTs: ", list);
      inProgressNfts = list;
    })
    .catch((err) => console.error("There was an error while trying to fetch the list of InProgress NFTs: ", err));

  return inProgressNfts;
}

// Get policy objects
export async function getListOfPolicyRoles() {
  let roles = [];

  await window.contract.get_policy()
    .then((response) => {
      console.log("Success! Roles fetched.");
      roles = response.roles;
    })
    .catch((err) => console.error(`There was an error while trying to fetch the policy roles`, err));

    return roles;
}

export function logout() {
  console.log("?")
  window.walletConnection.signOut()
  //window.location.replace(window.location.origin + window.location.pathname)               // reload page
}

export async function login() {
  window.walletConnection.requestSignIn((await getRealConfig(mode)).contractName)
}