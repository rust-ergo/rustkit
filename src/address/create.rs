use ergotree_ir::{ergo_tree::ErgoTree, chain::address::{Address, NetworkPrefix, AddressEncoder}};

pub fn convert_address_to_ergo_tree(address_str: &str) -> ErgoTree {
    let address: Address = convert_address_str_to_address(address_str);
    let ergo_tree: ErgoTree = address.script().unwrap();
    ergo_tree
}

pub fn convert_address_str_to_address(address_str: &str) -> Address {
    let address_encoder: AddressEncoder = AddressEncoder::new(NetworkPrefix::Mainnet);
    let address: Address = address_encoder.parse_address_from_str(address_str).unwrap();
    address
}