use crate::*;

#[near_bindgen]
impl Contract {
    /// Initiate buying of an NFT. 
    /// This function will initiate a cross-contract-call, and _buy_nft_from_vault()_ will do the actual moving of the NFT, in the FonoRoot minting contract
    pub fn buy_nft(&self, root_id: TokenId, minting_contract: AccountId) {
        log!("buy_nft() inside DAO contract started, root_id: {}, minting_contract: {}", root_id, minting_contract);

        let uniq_id = format!("{}-{}", minting_contract, root_id);
        // Check if the NFT exists in our system, it would be a problem if we would facilitate the buying of an NFT that is not connected to the DAO
        let tree_index = self.uniq_id_to_tree_index.get(&uniq_id.clone()).unwrap_or_else(|| {
            panic!("TreeIndex not found! Most likely root_id or contract is incorrect.");
        });

        let price = u128::from(self.income_tables.get(&tree_index).unwrap().price.unwrap());
        assert_eq!(
            env::attached_deposit(),
            price,
            "Exact price needs to be send to buy NFT"
        );

        let args = BuyArgs {
            root_id: root_id
        };
        let json_args = near_sdk::serde_json::to_string(&args).unwrap();
        let base64_args = json_args.clone().into_bytes();                
        
        let promise = Promise::new(minting_contract);
        
        let action = ActionCall {
            method_name: "buy_nft_from_vault".to_string(),
            args: base64_args.into(),
            deposit: U128(price + 10000000000000000000000),
            gas: U64(100000000000000),
        };

        log!("Prepairing cross-contract call...");
                
        promise.function_call(
            action.method_name.clone().into(),
            action.args.clone().into(),
            action.deposit.0,
            Gas(action.gas.0),
        )
        .then(ext_self::buy_nft_callback(
            tree_index,
            env::current_account_id(),
            0,
            Gas(50000000000000)
        ));
        
        log!("Initiating cross-contract call! Function inside DAO contract exiting...");

        // Should send back money if it fails! I guess in that case we wouldn't take the money in the first place, because the transaction would fail.
    }

    /// Callback that will run when the NFT was successfully moved to the new owner, the callback is updating the balances in the IncomeTable
    #[private]
    pub fn buy_nft_callback(
        &mut self, 
        #[callback_result] result: Result<bool, near_sdk::PromiseError>,
        tree_index: TreeIndex
    ) {
        assert_eq!(
            result.unwrap(),
            true,
            "Result should be true."
        );
        log!("Hello from buy_callback_test()!");
        log!("This NFT was bought: {} (TreeIndex)", tree_index);
        let mut the_income_table = self.income_tables.get(&tree_index).unwrap();
        //let catalogue_for_artist = self.catalogues.get(&the_income_table.owner).unwrap();
        //let the_catalogue_entry = catalogue_for_artist.get(&tree_index).unwrap().unwrap();
        the_income_table.total_income = the_income_table.total_income + u128::from(the_income_table.price.unwrap());
        the_income_table.current_balance = the_income_table.current_balance + u128::from(the_income_table.price.unwrap());
        self.income_tables.insert(&tree_index, &the_income_table);              // Would the same thing happen without this line?
    }
}