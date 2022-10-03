use ergo_lib::wallet::{Wallet, secret_key::SecretKey, ext_secret_key::ExtSecretKey, mnemonic::Mnemonic};
use ergotree_ir::{chain::{address::{Address, AddressEncoder, NetworkPrefix}, ergo_box::ErgoBox}};

use crate::{utils::format::remove_quotes, config::{file::Config}};


/// RustKit wallet
pub struct RustKitWallet {
    secret_key: SecretKey,
    pub index_0_address: String,
    pub wallet: Wallet,
    config: Config,
}

impl RustKitWallet {
    pub fn new(config: Config) -> Self {
        let mnemonic: String = config.get_mnemonic();
        let mnemonic_password: String = config.get_mnemonic_password();
        let seed = Mnemonic::to_seed(&mnemonic, &mnemonic_password);
        let extended_secret_key: ExtSecretKey = ExtSecretKey::derive_master(seed).unwrap();
        let secret_key: SecretKey = SecretKey::dlog_from_bytes(&extended_secret_key.secret_key_bytes()).unwrap();
        let cloned_key: SecretKey = secret_key.clone();
        let mut secret_keys_vec: Vec<SecretKey> = Vec::new();
        secret_keys_vec.push(cloned_key);
        let wallet: Wallet = Wallet::from_secrets(secret_keys_vec);
        let addr: Address = secret_key.get_address_from_public_image();
        match config.network.as_str() {
            "mainnet" => {
                let address_encoder: AddressEncoder = AddressEncoder::new(NetworkPrefix::Mainnet);
                let index_0_address: String = address_encoder.address_to_str(&addr);
                let index_0_address: String = remove_quotes(index_0_address);
                RustKitWallet { secret_key, index_0_address, wallet, config }
            },
            "testnet" => {
                let address_encoder: AddressEncoder = AddressEncoder::new(NetworkPrefix::Testnet);
                let index_0_address: String = address_encoder.address_to_str(&addr);
                let index_0_address: String = remove_quotes(index_0_address);
                RustKitWallet { secret_key, index_0_address, wallet, config }
            },
            _ => panic!("Invalid network"),
        }
    }

    /// Get sigma-rust address type from wallet
    pub fn get_address(&self) -> Address {
        let master_key: &SecretKey = &self.secret_key.clone();
        let master_key: SecretKey = master_key.to_owned();
        let address: Address = master_key.get_address_from_public_image();
        address
    }

    /// Get a p2pk address as a string from wallet
    pub fn get_p2pk_address(&self) -> String {
        return self.index_0_address.clone();
    }

    /// Get unspent boxes from explorer for wallet address
    pub fn get_input_boxes(&self) -> Option<Vec<ErgoBox>> {
        let bxs: Option<Vec<ErgoBox>> = ergo_rustkit_endpoints::get_unspent_boxes_for_address(self.index_0_address.as_str(), self.config.clone().explorer_url);
        bxs
    }
}