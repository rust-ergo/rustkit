use anyhow::Result;
use serde_json::Value;

use ergo_lib::ergotree_ir::chain::ergo_box::ErgoBox;

use crate::utils::consts::MAINNET_EXPLORER_API_BASE_URL;

fn request_unspent_boxes_for_address(address: &str) -> Result<Value> {
    let url: String = format!("{}/api/v1/boxes/unspent/byAddress/{}", MAINNET_EXPLORER_API_BASE_URL, address);
    let response: String = reqwest::blocking::get(&url)?.text()?;
    let json: Value = serde_json::from_str(&response)?;
    Ok(json)
}

pub fn get_unspent_boxes_for_address(address: &str) -> Option<Vec<ErgoBox>> {
    let json: Value = request_unspent_boxes_for_address(address).unwrap();
    let total: u64 = json["total"].as_u64().unwrap();
    if total == 0 {
        return None;
    }
    let boxes: &Vec<Value> = json["items"].as_array().unwrap();
    let mut box_vec: Vec<ErgoBox> = Vec::new();
    for b in boxes {
        let ergobox: ErgoBox = serde_json::from_value(b.clone()).unwrap();
        box_vec.push(ergobox);
    }
    return Some(box_vec);
}

fn request_current_height() -> Result<Value> {
    let url: String = format!("{}/api/v1/blocks?limit=1", MAINNET_EXPLORER_API_BASE_URL);
    let response: String = reqwest::blocking::get(&url)?.text()?;
    let json: Value = serde_json::from_str(&response)?;
    Ok(json)
}

pub fn get_current_height() -> u64 {
    let json: Value = request_current_height().unwrap();
    let height: u64 = json["total"].as_u64().unwrap();
    return height;
}