# Ergo RustKit

### Introduction

A SDK for building applications on the Ergo blockchain. Our goal is to create a library that creates a simple and easy to use interface for developers to build applications. Currently, the library is in the early stages of development and is not recommended for production use. The full roadmap can be found below!

### Setup

The Ergo RustKit is published on [crates.io](https://crates.io/). The crate can be found [here](https://crates.io/crates/ergo-rustkit).

```
ergo-rustkit = "0.2.0"
```

### Examples

##### Simple Send

```rust
let mut w: RustKitWallet = RustKitWallet::new("MNEMONIC", "MNEMONIC_PASSWORD");
w.update_index_0_address();

let mut tx: RustKitTransaction = RustKitTransaction::new("RECIPIENT ADDRESS", 100000000, 1100000, w);
tx.build();
tx.sign();
let resp: String = tx.submit().unwrap();
```

##### Send with token

```rust
let mut w: RustKitWallet = RustKitWallet::new("MNEMONIC", "MNEMONIC_PASSWORD");
w.update_index_0_address();

let mut tx: RustKitTransaction = RustKitTransaction::new("RECIPIENT ADDRESS", 100000000, 1100000, w);
tx.add_token("TOKEN ID", 1000);
tx.build();
tx.sign();
let resp: String = tx.submit().unwrap();
```

##### Mint new token

```rust
let mut w: RustKitWallet = RustKitWallet::new("MNEMONIC", "MNEMONIC_PASSWORD");
w.update_index_0_address();

let mut tx: RustKitTransaction = RustKitTransaction::new("RECIPIENT ADDRESS", 100000000, 1100000, w);
tx.mint_token("RustKit Mint", "Minted using Rustkit", 100, 2);
tx.build();
tx.sign();
let resp: String = tx.submit().unwrap();
```

##### Multi-Recipient Send

```rust
let mut w: RustKitWallet = RustKitWallet::new("MNEMONIC", "MNEMONIC_PASSWORD");
w.update_index_0_address();

let mut tx: RustKitTransaction = RustKitTransaction::new("RECIPIENT ADDRESS", 100000000, 1100000, w);
tx.add_token("TOKEN ID", 1000);
tx.add_reciever("SECOND RECIPIENT ADDRESS", 100000000, Some("TOKEN ID"), Some(1000));
tx.build();
tx.sign();
let resp: String = tx.submit().unwrap();
```
### Roadmap

- [X] Wallet Utils
  - [X] Get P2PK address
  - [X] Get boxes for Wallet
- [ ] Transaction Utils
  - [X] Ergo Only Transactions
  - [X] Ergo + Assets Transactions
    - [X] Send Assets
    - [X] Mint Assets
  - [ ] Set Data Inputs
  - [ ] Set Registers
  - [X] Multi-Recipient Transactions

### Projects using Ergo RustKit

Coming Soon...

### Contributing

Rust-Ergo is always open for contributions! If you would like to contribute, please open a PR and we will review it as soon as possible.