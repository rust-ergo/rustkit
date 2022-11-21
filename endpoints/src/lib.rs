use anyhow::Result;
use reqwest::blocking::{Client, Response};
use serde_json::Value;

pub mod utils;

use ergo_lib::ergotree_ir::chain::ergo_box::ErgoBox;
use ergo_chain_types::Header;
use ergo_lib::chain::ergo_state_context::Headers;

use crate::utils::consts::{MAINNET_EXPLORER_API_BASE_URL, MAINNET_NODE_URL};
use crate::utils::format::remove_quotes;

pub fn get_wallet_boxes(node_url: &str) -> Option<Vec<ErgoBox>> {
    let boxes_json: Result<Value> = request_wallet_boxes(node_url);
    if boxes_json.is_err() {
        return None;
    }
    let boxes_json: Value = boxes_json.unwrap();
    let boxes: Vec<ErgoBox> = serde_json::from_value(boxes_json).unwrap();
    Some(boxes)
} 

fn request_wallet_boxes(node_url: &str) -> Result<Value> {
    let url: String = format!("{}/wallet/boxes/unspent?minConfirmations=-1&maxConfirmations=-1&minInclusionHeight=0&maxInclusionHeight=-1", node_url);
    let response: String = reqwest::blocking::get(&url)?.text()?;
    let json: Value = serde_json::from_str(&response)?;
    Ok(json)
}

fn request_unspent_boxes_for_address(address: &str, explorer_url: Option<String>) -> Result<Value> {
    let base_url: String = match explorer_url {
        Some(url) => url,
        None => MAINNET_EXPLORER_API_BASE_URL.to_string(),
    };
    let url: String = format!("{}/api/v1/boxes/unspent/byAddress/{}", base_url, address);
    let response: String = reqwest::blocking::get(&url)?.text()?;
    let json: Value = serde_json::from_str(&response)?;
    Ok(json)
}

/// Returns on option of a vec of ErgoBoxes for an address on the mainnet
pub fn get_unspent_boxes_for_address(address: &str, explorer_url: Option<String>) -> Option<Vec<ErgoBox>> {
    let json: Value = request_unspent_boxes_for_address(address, explorer_url).unwrap();
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
    Some(box_vec)
}

pub fn get_current_height(node_url: Option<String>) -> Option<i64> {
    let url: String = match node_url {
        Some(url) => url,
        None => MAINNET_NODE_URL.to_string(),
    };
    let node_info: Option<Value> = get_node_info(Some(url));
    if node_info.is_none() {
        return None;
    }
    let node_info: Value = node_info.unwrap();
    let height: i64 = node_info["fullHeight"].as_i64().unwrap();
    Some(height)
}

/// Get the last 10 block headers
pub fn get_last_10_headers(node_url: Option<String>) -> Headers {
    let height: i64 = get_current_height(node_url.clone()).unwrap();
    let mut headers_vec: Vec<Header> = Vec::new();
    for i in 0..10 {
      let height: i64 = height - i;
      let header_id: String = get_header_id_by_height(height, node_url.clone()).unwrap();
      let header: Header = get_header_by_header_id(header_id, node_url.clone());
      headers_vec.push(header);
    }
    let headers: Headers = [headers_vec.get(0).unwrap().to_owned(), headers_vec.get(1).unwrap().to_owned(), headers_vec.get(2).unwrap().to_owned(), headers_vec.get(3).unwrap().to_owned(), headers_vec.get(4).unwrap().to_owned(), headers_vec.get(5).unwrap().to_owned(), headers_vec.get(6).unwrap().to_owned(), headers_vec.get(7).unwrap().to_owned(), headers_vec.get(8).unwrap().to_owned(), headers_vec.get(9).unwrap().to_owned()];
    headers
}

fn get_header_by_header_id(header_id: String, node_url: Option<String>) -> Header {
    let base_url: String = match node_url {
        Some(url) => url,
        None => MAINNET_NODE_URL.to_string(),
    };
    let url: String = format!("{}/blocks/{}/header", base_url, header_id);
    let resp: String = reqwest::blocking::get(url).unwrap().text().unwrap();
    let data: Value = serde_json::from_str(&resp).unwrap();
    let header: Header = serde_json::from_value(data).unwrap();
    header
}

fn get_header_id_by_height(height: i64, node_url: Option<String>) -> Result<String> {
    let base_url: String = match node_url {
        Some(url) => url,
        None => MAINNET_NODE_URL.to_string(),
    };
    let url: String = format!("{}/blocks/at/{}", base_url, height);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    let header_id: String = remove_quotes(data[0].to_string());
    Ok(header_id)
}

/// Submits the signed transaction to the network
pub fn submit(transaction_json: String, node_url: Option<String>) -> Result<String> {
    let base_url: String = match node_url {
        Some(url) => url,
        None => MAINNET_NODE_URL.to_string(),
    };
    let url: String = format!("{}/transactions", base_url);
    let client: Client = reqwest::blocking::Client::new();
    let response: Response = client.post(url)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .header("mode", "cors")
      .header("Access-Control-Allow-Origin", "*")
      .header("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept")
      .body(transaction_json)
      .send()?;
  
    let response_body: String = response.text()?;
    Ok(response_body)
}

pub fn get_node_info(node_url: Option<String>) -> Option<Value> {
    let node_info: Result<Value> = request_node_info(node_url);
    match node_info {
        Ok(info) => Some(info),
        Err(_) => None,
    }
}

fn request_node_info(node_url: Option<String>) -> Result<Value> {
    let base_url: String = match node_url {
        Some(url) => url,
        None => MAINNET_NODE_URL.to_string(),
    };
    let url: String = format!("{}/info", base_url);
    let resp: String = reqwest::blocking::get(url).unwrap().text().unwrap();
    let data: Value = serde_json::from_str(&resp).unwrap();
    let node_info: Value = serde_json::from_value(data).unwrap();
    Ok(node_info)
}