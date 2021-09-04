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
```bash
cargo run > read_rs.did

cargo build --release --target wasm32-unknown-unknown

ic-cdk-optimizer target/wasm32-unknown-unknown/release/read_rs.wasm -o ./read_rs_opt.wasm

```

## test
```bash
dfx canister --no-wallet --network ic call chu2x-jyaaa-aaaah-aaqra-cai get_now_index
(16_344 : nat64)

dfx canister --no-wallet --network ic call read_rs get_latest_version
(record { version = 17_011 : nat64 })

dfx canister --no-wallet --network ic call read_rs account_balance_pb '(record {account="073ca335431d6b6f6916068b5784a241730d2e3452ae650025b4bf7a975a81f0"})'
(record { e8s = 47_110_000 : nat64 })

dfx canister --no-wallet --network ic call read_rs block_pb '(507504:nat64)'(
  record {
    transaction = record {
      memo = 182_884_116_570_714_352 : nat64;
      created_at_time = record {
        timestamp_nanos = 1_630_776_083_125_587_176 : nat64;
      };
      transfer = variant {
        Send = record {
          to = "4dfa940def17f1427ae47378c440f10185867677109a02bc8374fc25b9dee8af";
          fee = record { e8s = 10_000 : nat64 };
          from = "4acd7ec9e1411fd23b2ed84fc173aacd220872408c1ff77a29961ab0f70d6ef6";
          amount = record { e8s = 32_616_390_000 : nat64 };
        }
      };
    };
    timestamp = record { timestamp_nanos = 1_630_776_086_437_763_870 : nat64 };
    parent_hash = opt record {
      inner = blob "\eaJ!\c8\17H\94\89\d8\d0z\e9\a0$.\12\d2\d8\0d?\c4\84\be\f3n\a2\8e\14u\0a\af)";
    };
  },
)

```