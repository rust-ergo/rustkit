# Send Token

This tutorial will show you how to send Erg and tokens to a single address.


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

All we have to do is add the token to the transaction. The first parameters is the token's ID (same token ID found on chain), while the second parameter is the amount of tokens to send.

```rust
tx.add_token("TOKEN_ID", 100);
```

### Final Code 

```rust
use ergo_rustkit::{transaction::create::{RustKitTransaction}, wallet::wallet::RustKitWallet, config::file::Config};

fn main() {
    let config_file: Config = Config::new("config.json");
    let w: RustKitWallet = RustKitWallet::new(config_file);

    let mut tx: RustKitTransaction = RustKitTransaction::new("RECIPIENT ADDRESS", 100000000, w);
    tx.add_token("TOKEN_ID", 100);
    tx.build();
    tx.sign();
    let resp: String = tx.submit().unwrap();
    println!("{}", resp);
}
```