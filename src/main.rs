mod proto;
use ledger_canister::{Block, EncodedBlock, BlockRes, AccountBalanceArgs, 
    account_identifier::AccountIdentifier, icpts::ICPTs, BlockHeight, TotalSupplyArgs
};
use dfn_core::{
    api::call_with_cleanup,
    over, over_async, over_init
};
use dfn_protobuf::{protobuf, ProtoBuf, ToProto};
use ic_types::CanisterId;
use dfn_candid::{candid, candid_one, CandidOne};
use serde::{
    de::{Deserializer, MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Serialize, Serializer,
};
use candid::{CandidType, Nat};
use prost::Message;


#[derive(Serialize, Deserialize, CandidType, Clone, Hash, Debug, PartialEq, Eq)]
pub struct InitArgs {
}

#[derive(Serialize, Deserialize, CandidType, Clone, Hash, Debug, PartialEq, Eq)]
pub struct SetArgs {
    pub input: u64,
}

impl ToProto for InitArgs {
    type Proto = proto::InitRequest;
    fn from_proto(_: Self::Proto) -> Result<Self, String> {
        Ok(InitArgs {})
    }

    fn to_proto(self) -> proto::InitRequest {
        proto::InitRequest {}
    }
}





static mut COUNTER: Option<Nat> = None;
const LEDGER : CanisterId = CanisterId::from_u64(2);

#[export_name = "canister_init"]
fn main() {
    over_init(|_: InitArgs| init())
}

fn init() {
    unsafe {
        COUNTER = Some(Nat::from(1)); 
    }
}

#[export_name = "canister_update increment"]
fn increment_() {
    over(candid_one, increment());
}

fn increment() {
    unsafe { 
        COUNTER.as_mut().unwrap().0 += 1;
    }
}

#[export_name = "canister_query get"]
fn get_() {
    over(candid_one, get());
}

fn get() -> candid::Nat {
    unsafe { COUNTER.as_mut().unwrap().clone() }
}

#[export_name = "canister_update set"]
fn set_() {
    over(candid_one, |SetArgs { input, } |  set(input),);
}

fn set(input: Nat) {
    unsafe {
        COUNTER.as_mut().unwrap().0 = input.0;
    }
}

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

async fn account_balance(account: AccountIdentifier) -> ICPTs {
    let result: Result<ICPTs, (Option<i32>, String)> = call_with_cleanup(
        LEDGER,
        "account_balance_pb",
        protobuf,
        AccountBalanceArgs { account }
    )
    .await;

    result.unwrap()
}