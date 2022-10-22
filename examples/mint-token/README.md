# Mint Token

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

All we have to do is add the token to the transaction. The first parameter is the name of the new token, while the second parameter is the token's description. The final two parameters are the token's initial supply and the number of decimals.

```rust
tx.mint_token("TOKEN_NAME", "TOKEN_DESCRIPTION", 100, 2);
```

### Final Code 

```rust
use ergo_rustkit::{transaction::create::{RustKitTransaction}, wallet::wallet::RustKitWallet, config::file::Config};

fn main() {
    let config_file: Config = Config::new("config.json");
    let w: RustKitWallet = RustKitWallet::new(config_file);

    let mut tx: RustKitTransaction = RustKitTransaction::new("RECIPIENT ADDRESS", 100000000, w);
    tx.mint_token("TOKEN_NAME", "TOKEN_DESCRIPTION", 100, 2);
    tx.build();
    tx.sign();
    let resp: String = tx.submit().unwrap();
    println!("{}", resp);
}
```