use dfn_core::{api::call_with_cleanup, over, over_async};
use dfn_protobuf::{protobuf, ProtoBuf};
use ic_types::CanisterId;
use candid::{CandidType, candid_method};
use dfn_candid::{candid, candid_one};
use ic_nns_handler_root::{
    common::{CanisterIdRecord, CanisterStatusResult}};
use ledger_canister::{Block, EncodedBlock, BlockRes, AccountBalanceArgs, 
    account_identifier::AccountIdentifier, icpts::ICPTs, BlockHeight
};


const IC_00: CanisterId = CanisterId::ic_00();
const REGISTRY_CANISTER_ID: CanisterId = CanisterId::from_u64(0);
const GOVERNANCE_CANISTER_ID: CanisterId = CanisterId::from_u64(1);
const LEDGER_CANISTER_ID: CanisterId = CanisterId::from_u64(2);
const ROOT_CANISTER_ID: CanisterId = CanisterId::from_u64(3);
const CYCLES_MINTING_CANISTER_ID: CanisterId = CanisterId::from_u64(4);
const LIFELINE_CANISTER_ID: CanisterId = CanisterId::from_u64(5);
const GENESIS_TOKEN_CANISTER_ID: CanisterId = CanisterId::from_u64(6);
const IDENTITY_CANISTER_ID: CanisterId = CanisterId::from_u64(7);
const NNS_UI_CANISTER_ID: CanisterId = CanisterId::from_u64(8);

//struct Actor

// management canister(virtual)
trait IC00 {
    fn canister_status() -> CanisterStatusResult;

}

trait Registry {

}



#[export_name = "canister_query balance"]
fn balance() {
    over_async(candid_one, | account: AccountBalanceArgs| {
        account_balance(account)
    })
}

#[candid_method(query, rename = "balance")]
async fn account_balance(account: AccountBalanceArgs) -> ICPTs {
    let result: Result<ICPTs, (Option<i32>, String)> = call_with_cleanup(
        LEDGER_CANISTER_ID,
        "account_balance_pb",
        protobuf,
        account
    )
    .await;

    result.unwrap()
}

#[export_name = "canister_query block"]
fn block() {
    over_async(candid_one, | bh: BlockHeight| {
        get_block_from_ledger(bh)
    })
}

#[candid_method(query, rename = "block")]
async fn get_block_from_ledger(block_height: BlockHeight) -> Block {
    let res: Result<BlockRes, (Option<i32>, String)> = call_with_cleanup(
        LEDGER_CANISTER_ID,
        "block_pb",
        protobuf,
        block_height
    )
    .await;

    let block = res.unwrap().0.unwrap().unwrap().decode().expect("unable to decode block");  
    block
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}