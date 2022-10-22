# Multi-Recipient Send

This tutorial will show you how to mint a new Ergo token.

### Step 1

We can take the same code that we used in the simple send tutorial as our starting point.

```rust
use ergo_rustkit::{transaction::create::{RustKitTransaction}, wallet::wallet::RustKitWallet, config::file::Config};

fn main() {
    let config_file: Config = Config::new("config.json");
    let w: RustKitWallet = RustKitWallet::new(config_file);

    let mut tx: RustKitTransaction = RustKitTransaction::new("RECIPIENT ADDRESS", 100000000, w);
    tx.build();
    tx.sign();
    let resp: String = tx.submit().unwrap();
    println!("{}", resp);
}
```

### Step 2

To add another reciever without tokens, we only need to set two parameters. The first parameter is the address, while the second parameter is the amount of Erg to send.

```rust
tx.add_reciever("SECOND RECIPIENT ADDRESS", 100000000, None, None);
```

### Step 3

To add another reciever with tokens, we need to set all parameters. The first two paramets are the same as before. The last two will specify the token ID and the amount of tokens to send.

```rust
tx.add_reciever("SECOND RECIPIENT ADDRESS", 100000000, Some("TOKEN ID"), Some(1000));
```

### Final Code 

```rust
use ergo_rustkit::{transaction::create::{RustKitTransaction}, wallet::wallet::RustKitWallet, config::file::Config};

fn main() {
    let config_file: Config = Config::new("config.json");
    let w: RustKitWallet = RustKitWallet::new(config_file);

    let mut tx: RustKitTransaction = RustKitTransaction::new("RECIPIENT ADDRESS", 100000000, w);
    tx.add_reciever("SECOND RECIPIENT ADDRESS", 100000000, None, None);
    tx.add_reciever("SECOND RECIPIENT ADDRESS", 100000000, Some("TOKEN ID"), Some(1000));
    tx.build();
    tx.sign();
    let resp: String = tx.submit().unwrap();
    println!("{}", resp);
}
```