use ergo_lib::wallet::{Wallet, secret_key::SecretKey, ext_secret_key::ExtSecretKey, mnemonic::Mnemonic, derivation_path::{DerivationPath, ChildIndexHardened, ChildIndexNormal}, ext_pub_key::ExtPubKey};
use ergotree_ir::{chain::{address::{Address, AddressEncoder, NetworkPrefix}, ergo_box::ErgoBox}};

use crate::{utils::format::remove_quotes, config::{file::Config}};


/// RustKit wallet
pub struct RustKitWallet {
    extended_secret_key: ExtSecretKey,
    pub index_0_address: String,
    addresses: Vec<Address>,
    pub wallet: Wallet,
    config: Config,
}

impl RustKitWallet {
    pub fn new(config: &Config) -> Self {
        let config: Config = config.to_owned();
        let mnemonic: String = config.get_mnemonic();
        let mnemonic_password: String = config.get_mnemonic_password();
        let seed = Mnemonic::to_seed(&mnemonic, &mnemonic_password);
        let extended_secret_key: ExtSecretKey = ExtSecretKey::derive_master(seed).unwrap();

        let path: DerivationPath = DerivationPath::new(ChildIndexHardened::from_31_bit(0).unwrap(), vec![ChildIndexNormal::normal(0).unwrap()]);
        let derived_extended_secret_key: ExtSecretKey = extended_secret_key.derive(path).unwrap();
        let secret_key: SecretKey = derived_extended_secret_key.secret_key();
        let public_key: ExtPubKey = derived_extended_secret_key.public_key().unwrap();
        let address: Address = Address::from(public_key);
        let wallet: Wallet = Wallet::from_secrets(vec![secret_key.clone()]);
        let addresses: Vec<Address> = Vec::new();
        match config.network.as_str() {
            "mainnet" => {
                let address_encoder: AddressEncoder = AddressEncoder::new(NetworkPrefix::Mainnet);
                let index_0_address: String = address_encoder.address_to_str(&address);
                let index_0_address: String = remove_quotes(index_0_address);
                RustKitWallet { extended_secret_key, index_0_address, addresses, wallet, config }
            },
            "testnet" => {
                let address_encoder: AddressEncoder = AddressEncoder::new(NetworkPrefix::Testnet);
                let index_0_address: String = address_encoder.address_to_str(&address);
                let index_0_address: String = remove_quotes(index_0_address);
                RustKitWallet { extended_secret_key, index_0_address, addresses, wallet, config }
            },
            _ => panic!("Invalid network"),
        }
    }

    pub fn init(&mut self) {
        let addresses: Vec<Address> = self.get_addresses();
        self.addresses = addresses;
    }

    pub fn get_eip_3_address(&self, index: u32) -> String {
        let address: Address = self.addresses.get(index as usize).unwrap().to_owned();
        match self.config.network.as_str() {
            "mainnet" => {
                let address_encoder: AddressEncoder = AddressEncoder::new(NetworkPrefix::Mainnet);
                let address: String = address_encoder.address_to_str(&address);
                let address: String = remove_quotes(address);
                address
            },
            "testnet" => {
                let address_encoder: AddressEncoder = AddressEncoder::new(NetworkPrefix::Testnet);
                let address: String = address_encoder.address_to_str(&address);
                let address: String = remove_quotes(address);
                address
            },
            _ => panic!("Invalid network"),
        }
    }

    /// Get sigma-rust address type from wallet
    pub fn get_addresses(&self) -> Vec<Address> {
        let mut address_vec: Vec<Address> = Vec::new();
        for i in 0..20 {
            let path: DerivationPath = DerivationPath::new(ChildIndexHardened::from_31_bit(0).unwrap(), vec![ChildIndexNormal::normal(i).unwrap()]);
            let derived_extended_secret_key: ExtSecretKey = self.extended_secret_key.derive(path).unwrap();
            let public_key: ExtPubKey = derived_extended_secret_key.public_key().unwrap();
            let address: Address = Address::from(public_key);
            address_vec.push(address);
        }
        return address_vec;
    }

    /// Get a p2pk address as a string from wallet
    pub fn get_p2pk_address(&self) -> String {
        return self.index_0_address.clone();
    }

    /// Get unspent boxes from explorer for wallet address
    pub fn get_input_boxes(&self) -> Option<Vec<ErgoBox>> {
        let bxs: Option<Vec<ErgoBox>> = ergo_rustkit_endpoints::get_unspent_boxes_for_address(self.index_0_address.as_str(), &self.config.clone().explorer_url);
        bxs
    }

    pub fn get_config(&self) -> Config {
        self.config.clone()
    }
}