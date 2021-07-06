use ledger_canister::{Block, EncodedBlock, BlockRes, AccountBalanceArgs, 
    account_identifier::AccountIdentifier, icpts::ICPTs, BlockHeight
};
use dfn_core::{api::call_with_cleanup, over, over_async};
use dfn_protobuf::{protobuf, ProtoBuf};
use ic_types::CanisterId;
use candid::{CandidType, candid_method};
use dfn_candid::{candid, candid_one};

static mut COUNTER: u64 = 1u64;
const LEDGER : CanisterId = CanisterId::from_u64(2);



#[export_name = "canister_update increment"]
fn increment() {
    over(candid, |()| increment_())
}

#[candid_method(update, rename = "increment")]
fn increment_() {
    unsafe {
        COUNTER += 1u64;
    }
}

#[export_name = "canister_query get"]
fn get() {
    over(candid, |()| -> u64 {
        get_()
    })
}

#[candid_method(query, rename = "get")]
fn get_() -> u64 {
    unsafe {
        COUNTER
    }
}

#[export_name = "canister_update sum2"]
fn combine2() {
    over(candid, |(a, b): (u64, u64)| -> u64 {
        sum2_(a, b)
    })
}

#[candid_method(update, rename = "sum2")]
fn sum2_(a: u64, b: u64) -> u64 {
    unsafe {
        let x = a + b;
        COUNTER = x;
        COUNTER
    }
}

#[export_name = "canister_update balance"]
fn balance() {
    over_async(candid_one, | account: AccountBalanceArgs| {
        account_balance(account)
    })
}

#[candid_method(update, rename = "balance")]
async fn account_balance(account: AccountBalanceArgs) -> ICPTs {
    let result: Result<ICPTs, (Option<i32>, String)> = call_with_cleanup(
        LEDGER,
        "account_balance_pb",
        protobuf,
        account
    )
    .await;

    result.unwrap()
}

#[export_name = "canister_update block"]
fn block() {
    over_async(candid_one, | bh: BlockHeight| {
        get_block_from_ledger(bh)
    })
}

#[candid_method(update, rename = "block")]
async fn get_block_from_ledger(block_height: BlockHeight) -> Block {
    let res: Result<BlockRes, (Option<i32>, String)> = call_with_cleanup(
        LEDGER,
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