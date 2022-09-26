# Ergo RustKit

### Example

##### Simple Send

```rust
let mut w: RustKitWallet = RustKitWallet::new("MNEMONIC", "MNEMONIC_PASSWORD");
w.update_index_0_address();

let mut tx: RustKitTransaction = RustKitTransaction::new("RECIPIENT ADDRESS", 100000000, 1100000);
tx.build(&w);
tx.sign(&w);
let resp: String = tx.submit().unwrap();
```

##### Send with token

```rust
let mut w: RustKitWallet = RustKitWallet::new("MNEMONIC", "MNEMONIC_PASSWORD");
w.update_index_0_address();

let mut tx: RustKitTransaction = RustKitTransaction::new("RECIPIENT ADDRESS", 100000000, 1100000);
tx.add_token("TOKEN ID", 1000);
tx.build(&w);
tx.sign(&w);
let resp: String = tx.submit().unwrap();
```

##### Multi-Recipient Send

```rust
let mut w: RustKitWallet = RustKitWallet::new("MNEMONIC", "MNEMONIC_PASSWORD");
w.update_index_0_address();

let mut tx: RustKitTransaction = RustKitTransaction::new("RECIPIENT ADDRESS", 100000000, 1100000);
tx.add_token("TOKEN ID", 1000);
tx.add_reciever("SECOND RECIPIENT ADDRESS", 100000000, Some("TOKEN ID"), Some(1000));
tx.build(&w);
tx.sign(&w);
let resp: String = tx.submit().unwrap();
```
### Roadmap

- [X] Wallet Utils
  - [X] Get P2PK address
  - [X] Get boxes for Wallet
- [ ] Transaction Utils
  - [X] Ergo Only Transactions
  - [ ] Ergo + Assets Transactions
    - [X] Send Assets
    - [ ] Mint Assets
  - [X] Multi-Recipient Transactions
  - [ ] Set data-inputs
  - [ ] Set registers

