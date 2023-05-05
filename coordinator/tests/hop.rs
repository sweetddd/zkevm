mod common;

use std::str::FromStr;
use crate::common::get_shared_state;
use ethers_core::abi::{AbiEncode, AbiParser, Token};
use ethers_core::abi::Tokenizable;
use ethers_core::types::{Address, H256};
use ethers_core::types::Bytes;
use ethers_core::types::TransactionReceipt;
use ethers_core::types::U256;
use ethers_core::types::U64;
use ethers_core::utils::hex;
use zkevm_common::json_rpc::jsonrpc_request;

#[ignore]
#[tokio::test]
async fn hop_deposit() {
    let abi = AbiParser::default()
        .parse(&[
            // hop-protocol
            "function sendToL2(uint256 chainId, address recipient, uint256 amount, uint256 amountOutMin, uint256 deadline, address relayer, uint256 relayerFee)",
        ])
        .expect("parse abi");

    let shared_state = await_state!();

    // hop-protocol deposit
    {
        let hop: Address = "0xb8901acB165ed027E32754E0FFe830802919727f"
            .parse()
            .unwrap();
        let chain_id = U256::from(99u64);
        let recipient = Address::zero();
        let amount = U256::from(0x174876e8000u64);
        let amount_out_min = U256::from(0x173c91838du64);
        let deadline = U256::MAX;
        let relayer = Address::zero();
        let relayer_fee = U256::zero();
        let calldata = abi
            .function("sendToL2")
            .unwrap()
            .encode_input(&[
                chain_id.into_token(),
                recipient.into_token(),
                amount.into_token(),
                amount_out_min.into_token(),
                deadline.into_token(),
                relayer.into_token(),
                relayer_fee.into_token(),
            ])
            .expect("calldata");

        let balance_before: U256 = jsonrpc_request(
            &shared_state.config.lock().await.l2_rpc_url,
            "eth_getBalance",
            (recipient, "latest"),
        )
        .await
        .expect("eth_getBalance");

        shared_state
            .transaction_to_l1(Some(hop), amount, calldata)
            .await
            .expect("receipt");
        sync!(shared_state);

        let balance_after: U256 = jsonrpc_request(
            &shared_state.config.lock().await.l2_rpc_url,
            "eth_getBalance",
            (recipient, "latest"),
        )
        .await
        .expect("eth_getBalance");

        let min_expected_balance = balance_before + amount_out_min;
        assert!(
            balance_after >= min_expected_balance,
            "ETH balance after hop deposit"
        );
    }

    finalize_chain!(shared_state);
}

#[ignore]
#[tokio::test]
async fn test1() {
    // address from, address to, uint256 value, uint256 fee, uint256 deadline, uint256 nonce, bytes data, bytes proof
    let abi = AbiParser::default()
        .parse(&[
            "event BlockSubmitted()",
            "event BlockFinalized(bytes32 blockHash)",
            "event MessageDispatched(address from, address to, uint256 value, uint256 fee, uint256 deadline, uint256 nonce, bytes data)",
            "event MessageDelivered(bytes32 id)",
            "function submitBlock(bytes)",
            "function finalizeBlock(bytes proof)",
            "function deliverMessageWithProof1(address from, address to, uint256 value, uint256 fee, uint256 deadline, uint256 nonce, bytes data, bytes proof)",
            "function deliverMessageWithProof8(address from, address to, uint256 value, uint256 fee, uint256 deadline, uint256 nonce, bytes data, bytes proof)",
            "function deliverMessageWithProof3(address from, address to, uint256 value)",
            "function deliverMessageWithProof4(address from, address to, uint256 value, uint256 fee)",
            "function deliverMessageWithProof5(address from, address to, uint256 value, uint256 fee, uint256 deadline)",
            "function deliverMessageWithProof6(address from, address to, uint256 value, uint256 fee, uint256 deadline, uint256 nonce)",
            "function deliverMessageWithProof7(address from, address to, uint256 value, uint256 fee, uint256 deadline, uint256 nonce, bytes data)",
            "function stateRoots(bytes32 blockHash) returns (bytes32)",
            "function importForeignBlock(uint256 blockNumber, bytes32 blockHash)",
            "function initGenesis(bytes32 blockHash, bytes32 stateRoot)",
            "function buildCommitment(bytes) returns (uint256[])",
            "function importForeignBridgeState(bytes, bytes)",
            "function multicall()",
            "function getTimestampForStorageRoot(bytes32 storageRoot) returns (uint256)",
        ])
        .expect("parse abi");

    pub struct MessageBeacon {
        pub id: H256,
        pub from: Address,
        pub to: Address,
        pub value: U256,
        pub fee: U256,
        pub deadline: U256,
        pub nonce: U256,
        pub calldata: Vec<u8>,
    }
    let msg = MessageBeacon{
        id : H256::zero(),
        from : Address::from_str("0x9f883b12fd0692714c2f28be6c40d3afdb9081d3").unwrap(),
        to: Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
        value: U256::from(999999999999999999u128),
        fee: U256::from(1),
        deadline: U256::from(18446744073709551615u128),
        nonce: U256::from(17341096732358035090u128),
        calldata: Vec::new(),
    };
    let calldata  =
        abi
        .function("deliverMessageWithProof4")
        .unwrap()
        .encode_input(&[
            msg.from.into_token(),
            msg.to.into_token(),
            msg.value.into_token(),
            msg.fee.into_token(),
            // msg.deadline.into_token(),
            // msg.nonce.into_token(),
            // Token::Bytes(msg.calldata),
            // Bytes::default().into_token(),
        ])
        .expect("calldata");

    let calldata_bytes = Bytes::from(calldata.clone());
    println!("sss {}",calldata_bytes)

}

#[ignore]
#[tokio::test]
async fn hop_cross_chain_message() {
    let abi = AbiParser::default()
        .parse(&[
            // hop-protocol
            "function swapAndSend(uint256 chainId, address recipient, uint256 amount, uint256 bonderFee, uint256 amountOutMin, uint256 deadline, uint256 destinationAmountOutMin, uint256 destinationDeadline)",
            "function commitTransfers(uint256 destinationChainId)",
            "function chainBalance(uint256)",
        ])
        .expect("parse abi");
    let calldata = abi
        .function("chainBalance")
        .unwrap()
        .encode_input(&[U256::from(99u64).into_token()])
        .expect("calldata");
    let get_chain_balance = serde_json::json!(
    {
        "to": "0xb8901acb165ed027e32754e0ffe830802919727f",
        "data": Bytes::from(calldata),
    }
    );

    let chain_id = U256::from(98u64);
    let shared_state = await_state!();

    sync!(shared_state);

    // balance on L1 hop bridge for L2 chain
    let chain_balance_before: U256 = jsonrpc_request(
        &shared_state.config.lock().await.l1_rpc_url,
        "eth_call",
        (&get_chain_balance, "latest"),
    )
    .await
    .expect("eth_call");

    {
        // withdraw from hop
        let hop: Address = "0x86cA30bEF97fB651b8d866D45503684b90cb3312"
            .parse()
            .unwrap();
        let recipient = Address::zero();
        let amount = U256::from(0x38d7ea4c68000u64);
        let bonder_fee = U256::from(0x54c89e3b2703u64);
        let amount_out_min = U256::from(0x330b7c6533df8u64);
        let deadline = U256::MAX;
        let destination_amount_out_min = amount_out_min - bonder_fee;
        let destination_deadline = U256::MAX;
        let calldata = abi
            .function("swapAndSend")
            .unwrap()
            .encode_input(&[
                chain_id.into_token(),
                recipient.into_token(),
                amount.into_token(),
                bonder_fee.into_token(),
                amount_out_min.into_token(),
                deadline.into_token(),
                destination_amount_out_min.into_token(),
                destination_deadline.into_token(),
            ])
            .expect("calldata");

        let tx_hash = shared_state
            .transaction_to_l2(Some(hop), amount, calldata, None)
            .await
            .expect("tx_hash");
        shared_state.mine().await;
        wait_for_tx!(tx_hash, &shared_state.config.lock().await.l2_rpc_url);
    }

    {
        // commit the hop stateroot and initiate L2 > L1 message
        let hop: Address = "0x83f6244bd87662118d96d9a6d44f09dfff14b30e"
            .parse()
            .unwrap();
        let calldata = abi
            .function("commitTransfers")
            .unwrap()
            .encode_input(&[chain_id.into_token()])
            .expect("calldata");
        let tx_hash_commit = shared_state
            .transaction_to_l2(Some(hop), U256::zero(), calldata, None)
            .await
            .expect("tx_hash_commit");
        shared_state.mine().await;
        wait_for_tx!(tx_hash_commit, &shared_state.config.lock().await.l2_rpc_url);
    }

    finalize_chain!(shared_state);

    {
        // verify that the L2 > L1 message was executed successfully
        let chain_balance_after: U256 = jsonrpc_request(
            &shared_state.config.lock().await.l1_rpc_url,
            "eth_call",
            (&get_chain_balance, "latest"),
        )
        .await
        .expect("eth_call");

        assert!(
            chain_balance_before > chain_balance_after,
            "hop-protocol chain balance"
        );
    }
}
