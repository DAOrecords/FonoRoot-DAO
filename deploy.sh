#!/bin/bash

export COUNCIL=["optr.testnet", "daorecords.testnet", "cryptovaibhav.testnet", "vandal.testnet", "testcranstest.testnet"]
export CONTRACT=mother-contract-test-2.soundsplash.testnet

ARGS='{"config": {"name": "MotherContract-Test2", "purpose": "Second Mother Contract Test. Testing registration, amonth other things. default_vote_policy was changed, so all council members can finalize adding Artist to minting contract alone.", "metadata":""}, "policy": '$COUNCIL'}'

echo "Contract is $CONTRACT"

# The NEAR command
near call $CONTRACT new --args '{"config": {"name": "MotherContract-Test2", "purpose": "Second Mother Contract Test. Testing registration, amonth other things. default_vote_policy was changed, so all council members can finalize adding Artist to minting contract alone.", "metadata":""}, "policy": ["optr.testnet", "daorecords.testnet", "cryptovaibhav.testnet", "vandal.testnet", "testcranstest.testnet"]}' --accountId optr.testnet
