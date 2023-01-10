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

/** This is for FonoRoot Minting contract calls (not the DAO contract) */
const { providers } = nearAPI;
// NEAR RPC
const mainnetProvider = new providers.JsonRpcProvider(
  "https://rpc.mainnet.near.org"
);

// NEAR testnet RPC
const testnetProvider = new providers.JsonRpcProvider(
  "https://rpc.testnet.near.org"
)

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
    viewMethods: ["get_policy", "get_last_proposal_id", "get_proposals", "get_in_progress_nfts", "get_catalogue", "get_single_income_table"],
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
      description: `MintRoot. ID: ${id}`,
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

// Create Revenue Table
export async function createRevenue(rootId, contract, revenueTable, price) {
  let success = false;

  const args = {
    proposal: {
      description: `Create Revenue Table for  uniqID: ${contract}-${rootId}`,
      kind: {
        CreateRevenueTable: {
          id: rootId,
          contract: contract,
          unsafe_table: revenueTable,
          price: utils.format.parseNearAmount(price)
        }
      }
    }
  };

  const gas = 100000000000000;
  const amount = utils.format.parseNearAmount("1");

  await window.contract.add_proposal(args, gas, amount)
    .then((msg) => {
      console.log("Success! (Create Revenue Table)", msg);
      success = true;
    })
    .catch((err) => console.error(`There was an error while creating the revenue table, uniqID ${contract}-${rootId}`, err));

  return success;
}

// Alter Revenue Table
export async function alterRevenue(treeIndex, revenueTable, price) {
  let success = false;

  const args = {
    proposal: {
      description: `Alter Revenue Table for treeIndex: ${treeIndex}`,
      kind: {
        AlterRevenueTable: {
          tree_index: treeIndex,
          unsafe_table: revenueTable,
          price: utils.format.parseNearAmount(price)
        }
      }
    }
  };

  const gas = 100000000000000;
  const amount = utils.format.parseNearAmount("1");

  await window.contract.add_proposal(args, gas, amount)
    .then((msg) => {
      console.log("Success! (Alter Revenue Table)", msg);
      success = true;
    })
    .catch((err) => console.error(`There was an error while altering the revenue table, treeIndex ${treeIndex}`, err));

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

// Get Catalogue for Artist
export async function getCatalogue(artist) {
  let catalogue = [];

  const args = {
    artist: artist
  }

  await window.contract.get_catalogue(args)
    .then((listAsArray) => {
      console.log(`Successfully got the Catalogue for ${artist}`, listAsArray);
      catalogue = listAsArray;
    })
    .catch((err) => console.error(`There was an error while trying to fetch the Catalogue for ${artist}: ${err}`));

  return catalogue;
}

// Fetch a single IncomeTable
export async function getSingleIncomeTable(treeIndex) {
  let incomeTable = null;

  const args = {
    id: treeIndex
  }

  await window.contract.get_single_income_table(args)
    .then((iTable) => {
      console.log(`Successfully got the IncomeTable for ${treeIndex}`, iTable);
      incomeTable = iTable;
    })
    .catch((err) => console.error(`There was an error while trying to fetch IncomeTable for ${treeIndex}: ${err}`));

  return incomeTable;
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

// Get metadata for an NFT
export async function getNftMetadata(contract, rootId) {
  let result = null;

  const provider = (mode === "development") ? testnetProvider : mainnetProvider; 
  await provider.query({
    request_type: "call_function",
    account_id: contract,
    method_name: "nft_token_details_for_list",
    args_base64: btoa(JSON.stringify({ token_list: [ rootId ] })),
    finality: "optimistic",
  })
    .then((rawResult) => {
      const response = JSON.parse(Buffer.from(rawResult.result).toString());
      result = response[0].metadata;
    })
    .catch((err) => console.error(`There was an error while trying to fetch metadata for ${rootId} on ${contract}`, err));

  return result;
}

export function logout() {
  console.log("?")
  window.walletConnection.signOut()
  //window.location.replace(window.location.origin + window.location.pathname)               // reload page
}

export async function login() {
  window.walletConnection.requestSignIn((await getRealConfig(mode)).contractName)
}