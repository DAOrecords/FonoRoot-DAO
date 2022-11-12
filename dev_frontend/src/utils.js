import { connect, Contract, keyStores, WalletConnection, utils, KeyPair } from 'near-api-js';
import * as nearAPI from "near-api-js";
const CryptoJS = require('crypto-js');

const mode = 'mainnet';       // 'mainnet' || 'development'

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
    viewMethods: ['nft_metadata', 'nft_token', 'nft_tokens_for_owner', 'nft_tokens', 'get_crust_key', 'get_next_buyable', 'view_guestbook_entries'],
    changeMethods: ['new_default_meta', 'new', 'mint_root', 'set_crust_key', 'buy_nft_from_vault', 'transfer_nft', 'create_guestbook_entry', 'withdraw', 'copy'],
  })
}



export function logout() {
  console.log("?")
  window.walletConnection.signOut()
  //window.location.replace(window.location.origin + window.location.pathname)               // reload page
}

export async function login() {
  window.walletConnection.requestSignIn((await getRealConfig(mode)).contractName)
}