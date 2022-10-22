use anyhow::Result;
use ergo_lib::chain::transaction::Transaction;

use crate::config::file::Config;

pub fn submit(tx: Transaction, config: Config) -> Result<String> {
    let transaction_json: String = transaction_to_json(tx);
    let resp: Result<String> = ergo_rustkit_endpoints::submit(transaction_json, &config.node_url);
    resp
}

pub fn transaction_to_json(tx: Transaction) -> String {
    let transaction_json: String = serde_json::to_string(&tx).unwrap();
    transaction_json
}