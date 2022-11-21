use ergotree_ir::{ergo_tree::ErgoTree, chain::address::{Address, NetworkPrefix, AddressEncoder}};

/// Convert an address to ErgoTree
pub fn convert_address_to_ergo_tree(address_str: &str) -> ErgoTree {
    let address: Address = convert_address_str_to_address(address_str);
    let ergo_tree: ErgoTree = address.script().unwrap();
    ergo_tree
}

/// Convert a string to an address
pub fn convert_address_str_to_address(address_str: &str) -> Address {
    let address_encoder: AddressEncoder = AddressEncoder::new(NetworkPrefix::Testnet);
    let address: Address = address_encoder.parse_address_from_str(address_str).unwrap();
    address
}

pub struct RustKitAddress {
    pub address: Address,
    pub address_str: String,
    pub script: ErgoTree,
}

impl RustKitAddress {
    pub fn recreate_from_str(address_str: &str) -> Self {
        let address_encoder: AddressEncoder = AddressEncoder::new(NetworkPrefix::Testnet);
        let address: Address = address_encoder.parse_address_from_str(address_str).unwrap();
        let script: ErgoTree = address.script().unwrap();
        RustKitAddress { address, address_str: address_str.to_string(), script }
    }
}