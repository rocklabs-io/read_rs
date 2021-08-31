# canister_rs

Welcome to your new canister_rs project and to the internet computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with canister_rs, see the following documentation available online:

- [Quick Start](https://sdk.dfinity.org/docs/quickstart/quickstart-intro.html)
- [SDK Developer Tools](https://sdk.dfinity.org/docs/developers-guide/sdk-guide.html)
- [Motoko Programming Language Guide](https://sdk.dfinity.org/docs/language-guide/motoko.html)
- [Motoko Language Quick Reference](https://sdk.dfinity.org/docs/language-guide/language-manual.html)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd canister_rs/
dfx help
dfx config --help
```


## build
```sh
cargo run > read_rs.did

cargo build --release --target wasm32-unknown-unknown

ic-cdk-optimizer target/wasm32-unknown-unknown/release/read_rs.wasm -o ./read_rs_opt.wasm

```

## run

start the runtime:
```sh
sudo dfx start --clean

```

go to the local-dev
```sh
cd local-dev/

sudo dfx canister --no-wallet create --all

dfx identity use minting
MINTING_ACCOUNT=\"$(dfx ledger account-id)\"
MINTING_PRINCIPAL="principal \"$(dfx identity get-principal)\""

dfx identity use alice
ALICE_ACCOUNT=\"$(dfx ledger account-id)\"
ALICE_PRINCIPAL="principal \"$(dfx identity get-principal)\""

dfx identity use admin
ADMIN_ACCOUNT=\"$(dfx ledger account-id)\"
ADMIN_PRINCIPAL="principal \"$(dfx identity get-principal)\""

AMOUNT=100_000_000_000_000
dfx canister --no-wallet install 3ledger --argument "record {minting_account=$MINTING_ACCOUNT; initial_values=vec {record{$ADMIN_ACCOUNT;record{e8s=$AMOUNT:nat64;}}}; max_message_size_bytes=null;transaction_window=opt record {secs=300:nat64;nanos=0:nat32};archive_options=null;send_whitelist=vec{};}"

dfx canister --no-wallet call 3ledger send_dfx "record {memo=0:nat64;amount=record{e8s=1000000000000:nat64};fee=record{e8s=10000:nat64};from_subaccount=null;to=$ALICE_ACCOUNT;created_at_time=null}"
```

go to the read_rs
```sh
sudo dfx canister --no-wallet create --all

sudo dfx canister --no-wallet install read_rs 

ADMIN_ACCOUNT=\"$(dfx ledger account-id)\"

dfx canister --no-wallet call read_rs balance "(record { account=$ADMIN_ACCOUNT})"
(record { e8s = 100_000_000_000_000 })

dfx canister --no-wallet call read_rs get
(1)

dfx canister --no-wallet call read_rs block '(0)'
(
  record {
    transaction = record {
      memo = 0;
      created_at_time = record { timestamp_nanos = 1_625_564_074_933_651_000 };
      transfer = variant {
        Mint = record {
          to = "02a0703aa42200bf7e9702b652f685a42d2c0b2aba84fcee1a86e4a56ca4ef10";
          amount = record { e8s = 100_000_000_000_000 };
        }
      };
    };
    timestamp = record { timestamp_nanos = 1_625_564_074_933_651_000 };
    parent_hash = null;
  },
)

dfx canister --no-wallet call read_rs block '(1)'
(
  record {
    transaction = record {
      memo = 0;
      created_at_time = record { timestamp_nanos = 1_625_566_737_203_904_000 };
      transfer = variant {
        Send = record {
          to = "ca5aa2bfed7d6b28723605d870578397f3c65df5c0a8a9de51af7dd5e6c22638";
          fee = record { e8s = 10_000 };
          from = "02a0703aa42200bf7e9702b652f685a42d2c0b2aba84fcee1a86e4a56ca4ef10";
          amount = record { e8s = 1_000_000_000_000 };
        }
      };
    };
    timestamp = record { timestamp_nanos = 1_625_566_737_203_904_000 };
    parent_hash = opt record {
      inner = blob "\a0\05-\e2D\13K\d6>\c8\a2\85h\faM=\8b\dc\b1\a7r\e5\d5\d7U\9f\c0\c2\1f\df\f9^";
    };
  },
)

dfx canister --no-wallet call read_rs balance "(record { account=$ADMIN_ACCOUNT})"
(record { e8s = 98_999_999_990_000 })

```