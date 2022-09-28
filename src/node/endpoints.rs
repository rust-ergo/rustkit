use crate::{utils::format::remove_quotes, explorer::endpoints::get_current_height};

use ergo_chain_types::Header;
use ergo_lib::chain::ergo_state_context::Headers;
use serde_json::{Value};
use anyhow::{Result};

const NODE_URL: &str = "http://13.56.77.38:9053";

/// Get the last 10 block headers
pub fn get_last_10_headers() -> Headers {
    let height: u32 = get_current_height() as u32;
    let mut headers_vec: Vec<Header> = Vec::new();
    for i in 0..10 {
      let height: u32 = height - i;
      let header_id: String = get_header_id_by_height(height).unwrap();
      let header: Header = get_header_by_header_id(header_id);
      headers_vec.push(header);
    }
    let headers: Headers = [headers_vec.get(0).unwrap().to_owned(), headers_vec.get(1).unwrap().to_owned(), headers_vec.get(2).unwrap().to_owned(), headers_vec.get(3).unwrap().to_owned(), headers_vec.get(4).unwrap().to_owned(), headers_vec.get(5).unwrap().to_owned(), headers_vec.get(6).unwrap().to_owned(), headers_vec.get(7).unwrap().to_owned(), headers_vec.get(8).unwrap().to_owned(), headers_vec.get(9).unwrap().to_owned()];
    headers
}

fn get_header_by_header_id(header_id: String) -> Header {
    let url: String = format!("{}/blocks/{}/header", NODE_URL, header_id);
    let resp: String = reqwest::blocking::get(url).unwrap().text().unwrap();
    let data: Value = serde_json::from_str(&resp).unwrap();
    let header: Header = serde_json::from_value(data).unwrap();
    header
}

fn get_header_id_by_height(height: u32) -> Result<String> {
    let url: String = format!("{}/blocks/at/{}", NODE_URL, height);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    let data: Value = serde_json::from_str(&resp)?;
    let header_id: String = remove_quotes(data[0].to_string());
    Ok(header_id)
}