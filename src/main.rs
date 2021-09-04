use dfn_core::{api::{call_with_cleanup, id, caller, canister_cycle_balance, canister_status}, over, over_async};
use dfn_protobuf::{protobuf, ProtoBuf};
use candid::{CandidType, Deserialize, candid_method};
use dfn_candid::{candid, candid_one};
use ic_registry_transport::pb::v1::{RegistryGetChangesSinceRequest, RegistryGetChangesSinceResponse, CertifiedResponse, RegistryGetValueRequest, RegistryGetValueResponse, RegistryGetLatestVersionResponse};
use ic_nns_handler_root::{
    common::{CanisterIdRecord, CanisterStatusResult}};
use ledger_canister::{Block, EncodedBlock, BlockRes, AccountBalanceArgs, 
    account_identifier::AccountIdentifier, icpts::ICPTs, BlockHeight
};
use ic_base_types::{CanisterId, PrincipalId, PrincipalIdParseError, PrincipalIdBlobParseError, CanisterIdError};
use std::convert::TryInto;
use ic_cdk_macros::*;


const CRC_LENGTH_IN_BYTES: usize = 4;

const REGISTRY_CANISTER_ID: CanisterId = CanisterId::from_u64(0);
const GOVERNANCE_CANISTER_ID: CanisterId = CanisterId::from_u64(1);
const LEDGER_CANISTER_ID: CanisterId = CanisterId::from_u64(2);
const ROOT_CANISTER_ID: CanisterId = CanisterId::from_u64(3);
const CYCLES_MINTING_CANISTER_ID: CanisterId = CanisterId::from_u64(4);
const LIFELINE_CANISTER_ID: CanisterId = CanisterId::from_u64(5);
const GENESIS_TOKEN_CANISTER_ID: CanisterId = CanisterId::from_u64(6);
const IDENTITY_CANISTER_ID: CanisterId = CanisterId::from_u64(7);
const NNS_UI_CANISTER_ID: CanisterId = CanisterId::from_u64(8);


// regestry
#[update]
#[candid_method(update)]
async fn get_changes_since(req: RegistryGetChangesSinceRequest) -> RegistryGetChangesSinceResponse {
    let result: Result<RegistryGetChangesSinceResponse, (Option<i32>, String)> = call_with_cleanup(
        REGISTRY_CANISTER_ID, 
        "get_changes_since", 
        protobuf, 
        req
    )
    .await;

    result.unwrap()
 }

#[update]
#[candid_method(update)]
async fn get_certified_changes_since(req: RegistryGetChangesSinceRequest) -> CertifiedResponse {
    let result: Result<CertifiedResponse, (Option<i32>, String)> = call_with_cleanup(
        REGISTRY_CANISTER_ID, 
        "get_certified_changes_since", 
        protobuf, 
        req
    )
    .await;

    result.unwrap()
}

#[update]
#[candid_method(update)]
async fn get_value(req: RegistryGetValueRequest) -> RegistryGetValueResponse {
    let result: Result<RegistryGetValueResponse, (Option<i32>, String)> = call_with_cleanup(
        REGISTRY_CANISTER_ID, 
        "get_value", 
        protobuf, 
        req
    )
    .await;

    result.unwrap()
}

#[update]
#[candid_method(update)]
async fn get_latest_version() -> RegistryGetLatestVersionResponse {
    let result: Result<RegistryGetLatestVersionResponse, (Option<i32>, String)> = call_with_cleanup(
        REGISTRY_CANISTER_ID, 
        "get_latest_version", 
        protobuf, 
        ()
    )
    .await;

    result.unwrap()
}

#[update]
#[candid_method(update)]
async fn get_latest_version_certified() -> CertifiedResponse {
    let result: Result<CertifiedResponse, (Option<i32>, String)> = call_with_cleanup(
        REGISTRY_CANISTER_ID, 
        "get_latest_version_certified", 
        protobuf, 
        ()
    )
    .await;

    result.unwrap()
}

#[update]
#[candid_method(update)]
async fn get_latest_version_certified() -> CertifiedResponse {
    let result: Result<CertifiedResponse, (Option<i32>, String)> = call_with_cleanup(
        REGISTRY_CANISTER_ID, 
        "get_latest_version_certified", 
        protobuf, 
        ()
    )
    .await;

    result.unwrap()
}








// governance
// ledger
// root
// cycles-minting
// lifeline
// genesis-token
// identity
// nns-ui

#[export_name = "canister_query balance"]
fn balance() {
    over_async(candid_one, | account: AccountBalanceArgs| {
        account_balance(account)
    })
}

#[candid_method(query, rename = "balance")]
async fn account_balance(account: AccountBalanceArgs) -> ICPTs {
    let ledger: CanisterId = canister_from_str("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();

    let result: Result<ICPTs, (Option<i32>, String)> = call_with_cleanup(
        ledger,
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
    let ledger: CanisterId = canister_from_str("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();

    let res: Result<BlockRes, (Option<i32>, String)> = call_with_cleanup(
        ledger,
        "block_pb",
        protobuf,
        block_height
    )
    .await;

    let block = res.unwrap().0.unwrap().unwrap().decode().expect("unable to decode block");  
    block
}

#[export_name = "canister_query now_index"]
fn now_index() {
    over_async(candid, |()| {
        now_index_()
    })
}

#[candid_method(query, rename = "now_index")]
async fn now_index_() -> (u64, u64) {
    let test_canister_id: CanisterId = canister_from_str("ywrdt-7aaaa-aaaah-qaaaa-cai").unwrap();

    let result: Result<(u64, u64), (Option<i32>, String)> = call_with_cleanup(
        test_canister_id,
        "get_now_index",
        candid_one,
        (),
    )
    .await;

    result.unwrap()
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}

fn canister_from_str(input: &str) -> Result<CanisterId, CanisterIdError> {
    let principal_id =
        principal_from_str(input).map_err(CanisterIdError::PrincipalIdParseError)?;
    CanisterId::new(principal_id)
}

fn principal_from_str(input: &str) -> Result<PrincipalId, PrincipalIdParseError> {
    // Strategy: Parse very liberally, then pretty-print and compare output.
    // This is both simpler and yields better error messages.
    let mut s = input.to_string();
    s.make_ascii_lowercase();
    s.retain(|c| c.is_ascii_alphanumeric());
    match base32::decode(base32::Alphabet::RFC4648 { padding: false }, &s) {
        Some(mut bytes) => {
            if bytes.len() < CRC_LENGTH_IN_BYTES {
                return Err(PrincipalIdParseError::TooShort);
            }
            if bytes.len() > PrincipalId::MAX_LENGTH_IN_BYTES + CRC_LENGTH_IN_BYTES {
                return Err(PrincipalIdParseError::TooLong);
            }
            let result =
                try_from(&bytes.split_off(CRC_LENGTH_IN_BYTES)[..]).unwrap();
            let expected = format!("{}", result);
            if input != expected {
                return Err(PrincipalIdParseError::Wrong { expected });
            }
            Ok(result)
        }
        None => Err(PrincipalIdParseError::NotBase32),
    }
}

fn try_from(blob: &[u8]) -> Result<PrincipalId, PrincipalIdBlobParseError> {
    // if blob.len() != PrincipalId::MAX_LENGTH_IN_BYTES {
    //     return Err(PrincipalIdBlobParseError::TooLong(blob.len()));
    // }
    let mut data = [0u8; PrincipalId::MAX_LENGTH_IN_BYTES];
    data[..blob.len()].copy_from_slice(&blob[..]);
    Ok(PrincipalId::new(blob.len(), data))
}