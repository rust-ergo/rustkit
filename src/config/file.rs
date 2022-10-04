use std::fs;
use serde_json::Value;

#[derive(Clone)]
pub struct Config {
    mnemonic: String,
    mnemonic_password: String,
    pub network: String,
    pub node_url: Option<String>,
    pub explorer_url: Option<String>,
}

impl Config {
    pub fn new(path: &str) -> Config {
        let file_contents: String = fs::read_to_string(path).expect("Cannot load config file");
        let json: Value = serde_json::from_str(&file_contents).expect("Cannot parse config file");
        let mut node_url: Option<String> = match json["nodeUrl"].is_null() {
            true => None,
            false => Some(json["nodeUrl"].as_str().unwrap().to_string()),
        };
        let mut explorer_url: Option<String> = match json["explorerUrl"].is_null() {
            true => None,
            false => Some(json["explorerUrl"].as_str().unwrap().to_string()),
        };
        if node_url == Some("".to_string()) {
            node_url = None;
        }
        if explorer_url == Some("".to_string()) {
            explorer_url = None;
        }
        Config {
            mnemonic: json["mnemonic"].as_str().expect("Cannot parse mnemonic").to_owned(),
            mnemonic_password: json["mnemonicPassword"].as_str().expect("Cannot parse mnemonic password").to_owned(),
            network: json["network"].as_str().expect("Cannot parse network").to_owned(),
            node_url: node_url,
            explorer_url: explorer_url
        }
    }

    pub fn get_mnemonic(&self) -> String {
        self.mnemonic.clone()
    }

    pub fn get_mnemonic_password(&self) -> String {
        self.mnemonic_password.clone()
    }
}