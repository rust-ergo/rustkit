# Simple Send

This tutorial will show you how to send Erg to a single address.


### Step 1

Add the dependency to your Cargo.toml file:

```toml
[dependencies]
rustkit = "0.4.0"
```

### Step 2

Create the config file. The `mnemonic` is your seed phrase and the `mnemonicPassword` is the seed phrase password. The `network` is the network you want to use. The network can be mainnet, or testnet. The `nodeUrl` and `explorerUrl` are optional and can be left out. If they are left out, the default node url and explorer url will be used. Otherwise, you can specify your own node url and explorer url.

```json
{
    "mnemonic": "MNEMONIC PHRASE",
    "mnemonicPassword": "MNEMONIC PASSWORD",
    "network": "mainnet",
    "nodeUrl": "",
    "explorerUrl": ""
}
```

### Step 3

Import the rustkit crate:

```rust
use ergo_rustkit::{transaction::create::{RustKitTransaction}, wallet::wallet::RustKitWallet, config::file::Config};
```

### Step 4

Import the config file. Create a new config object passing in the path to your config file.

```rust
let config_file: Config = Config::new("config.json");
```

### Step 5

Create the RustKit wallet, passing in the config file.

```rust
let w: RustKitWallet = RustKitWallet::new(config_file);
```

### Step 6

Create the RustKit transaction. Replace `RECIPIENT ADDRESS` with the address you want to send Erg to. The amount is in nanoErgs. The first amount is the send amount. Finally, add your wallet to the transaction.

```rust
let mut tx: RustKitTransaction = RustKitTransaction::new("RECIPIENT ADDRESS", 100000000, w);
```

### Step 7

Build and sign the transaction.

```rust
tx.build();
tx.sign();
```

### Step 8

Submit the transaction to the network and print the response from the explorer. The explorer will return the transaction ID if the transaction was successfully submitted.

```rust
let resp: String = tx.submit().unwrap();
println!("{}", resp);
```

### Final Code

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