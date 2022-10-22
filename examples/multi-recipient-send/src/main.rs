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