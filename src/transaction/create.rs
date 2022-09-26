use anyhow::Result;
use ergo_chain_types::{Header, PreHeader, BlockId, EcPoint, Votes};
use ergo_lib::{ergotree_ir::chain::ergo_box::ErgoBox, wallet::{box_selector::{SimpleBoxSelector, BoxSelection, BoxSelector}, tx_builder::TxBuilder, signing::TransactionContext, multi_sig::TransactionHintsBag}, chain::{ergo_box::box_builder::ErgoBoxCandidateBuilder, transaction::{unsigned::UnsignedTransaction, Transaction}, ergo_state_context::ErgoStateContext}};
use ergotree_ir::{chain::{ergo_box::{box_value::BoxValue, ErgoBoxCandidate}, token::{Token, TokenAmount, TokenId}, address::{Address, AddressEncoder, NetworkPrefix}}, ergo_tree::ErgoTree};

use explorer::endpoints::get_current_height;
use reqwest::blocking::{Client, Response};
use wallet::wallet::RustKitWallet;

use crate::{explorer, node, utils::consts::MAINNET_EXPLORER_API_BASE_URL};
use crate::wallet;

struct Reciver {
    address: String,
    value: u64,
    tokens: Option<Vec<Token>>,
}

impl Reciver {
    fn new(address: String, value: u64, tokens: Option<Vec<Token>>) -> Self {
        Reciver {
            address,
            value,
            tokens,
        }
    }
}

pub struct RustKitTransaction { 
    reciever: String,
    alt_recievers: Option<Vec<Reciver>>,
    value: u64,
    fee: u64,
    tokens: Option<Vec<Token>>,
    input_boxes: Option<Vec<ErgoBox>>,
    data_boxes: Option<Vec<ErgoBox>>,
    unsigned: Option<UnsignedTransaction>,
    signed: Option<Transaction>,
}

impl RustKitTransaction {
    pub fn new(receiver_address: &str, nano_erg_amount: u64, fee_amount: u64) -> Self {
        let tx = RustKitTransaction {
            reciever: receiver_address.to_owned(),
            alt_recievers: None,
            value: nano_erg_amount,
            fee: fee_amount,
            tokens: None,
            input_boxes: None,
            data_boxes: None,
            unsigned: None,
            signed: None,
        };
        return tx;
    }

    pub fn build(&mut self, wallet: &RustKitWallet) {
        let height: u32 = get_current_height() as u32;
        let input_boxes_explorer: Option<Vec<ErgoBox>> = wallet.get_input_boxes();
        if input_boxes_explorer.is_none() {
            panic!("No input boxes found for address: {}", wallet.index_0_address);
        }
        let input_boxes_explorer: Vec<ErgoBox> = input_boxes_explorer.unwrap();
        self.input_boxes = Some(input_boxes_explorer.clone());
        let mut input_boxes_raw_value: u64 = 0; 
        let mut input_boxes_raw_tokens: Vec<Token> = Vec::new();
        for box_ in input_boxes_explorer.clone().iter() {
            input_boxes_raw_value += box_.value.as_u64();
            let box_tokens = box_.tokens.clone();
            if box_tokens.is_some() {
                let box_tokens = box_tokens.unwrap();
                for token in box_tokens.iter() {
                    input_boxes_raw_tokens.push(token.clone());
                }
            }
        }

        let tx_output_boxes: Vec<ErgoBoxCandidate> = Self::create_output_candidates(self);
        let mut total_output_value: u64 = 0;
        let mut needed_tokens: Vec<Token> = Vec::new();
        for opt in &tx_output_boxes {
            total_output_value += opt.value.as_u64().to_owned();
            if opt.tokens.is_some() {
                let tokens = opt.tokens.clone().unwrap();
                for token in tokens.iter() {
                    needed_tokens.push(token.clone());
                }
            }
        }

        let box_value_needed: BoxValue = BoxValue::try_from(total_output_value + self.fee).unwrap();
        let tx_input_boxes: BoxSelection<ErgoBox> = Self::get_input_boxes(self.input_boxes.clone().unwrap(), box_value_needed, &Some(needed_tokens));

        let data_boxes: Vec<ErgoBox> = Vec::new();
        self.data_boxes = Some(data_boxes);

        let fee_amount: BoxValue = BoxValue::new(self.fee).unwrap();

        let change_address: Address = convert_address_str_to_address(wallet.index_0_address.as_str());

        let transaction_builder: TxBuilder<ErgoBox> = TxBuilder::new(tx_input_boxes, tx_output_boxes, height, fee_amount, change_address);
        let unsigned: UnsignedTransaction = transaction_builder.build().unwrap();
        self.unsigned = Some(unsigned);
    }

    pub fn sign(&mut self, wallet: &RustKitWallet) {
        let last_10_headers: [Header; 10] = node::endpoints::get_last_10_headers();
        let preheader: PreHeader = create_preheader(&last_10_headers[0]);
        let transaction_context: TransactionContext<UnsignedTransaction> = TransactionContext::new(self.unsigned.clone().unwrap(), self.input_boxes.clone().unwrap(), self.data_boxes.clone().unwrap()).unwrap();
        let state_context: ErgoStateContext = ErgoStateContext::new(preheader, last_10_headers);
        let transaction_hints: TransactionHintsBag = wallet.wallet.generate_commitments(transaction_context.clone(), &state_context).unwrap();
        let signed_transaction: Transaction = wallet.wallet.sign_transaction(transaction_context, &state_context, Some(&transaction_hints)).unwrap();
        self.signed = Some(signed_transaction.clone());
    }

    pub fn submit(&mut self) -> Result<String> {
        let transaction_json: String = self.get_signed_transaction_as_json();
        let url: String = format!("{}/api/v1/mempool/transactions/submit", MAINNET_EXPLORER_API_BASE_URL);
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
        return Ok(response_body);
    }

    pub fn add_reciever(&mut self, receiver_address: &str, nano_erg_amount: u64, tokens_id: Option<&str>, tokens_amount: Option<u64>) {
        let mut tokens: Option<Vec<Token>> = None;
        if tokens_id.is_some() {
            let token_id = tokens_id.unwrap();
            let token_amount = tokens_amount.unwrap();
            let id_base16: Vec<u8> = base16::decode(&token_id).unwrap();
            let id_base64: String = base64::encode(&id_base16);
            let tk = Token {
                token_id: TokenId::from_base64(&id_base64).unwrap(),
                amount: TokenAmount::try_from(token_amount as u64).unwrap(),
            };
            tokens = Some(vec![tk]);
        }

        let rec: Reciver = Reciver::new(receiver_address.to_owned(), nano_erg_amount, tokens);
        if self.alt_recievers.is_none() {
            self.alt_recievers = Some(vec![rec]);
        } else {
            let recievers: Vec<Reciver> = vec![rec];
            self.alt_recievers = Some(recievers);
        }
    }

    pub fn add_token(&mut self, token_id: &str, amount: u64) {
        let id_base16: Vec<u8> = base16::decode(token_id).unwrap();
        let id_base64: String = base64::encode(&id_base16);
        let tk = Token {
            token_id: TokenId::from_base64(&id_base64).unwrap(),
            amount: TokenAmount::try_from(amount as u64).unwrap(),
        };
        let token_vec: &Option<Vec<Token>> = &self.tokens;
        let token_vec: Option<Vec<Token>> = token_vec.to_owned();
        if token_vec.is_none() {
            let mut tokens: Vec<Token> = Vec::new();
            tokens.push(tk);
            self.tokens = Some(tokens);
        } else {
            let mut tokens: Vec<Token> = token_vec.unwrap();
            tokens.push(tk);
            self.tokens = Some(tokens);
        }
    }

    pub fn mint_token(&mut self) {
        // TODO
        todo!();
    }

    fn get_input_boxes(input_boxes: Vec<ErgoBox>, nano_erg_amount: BoxValue, tokens: &Option<Vec<Token>>) -> BoxSelection<ErgoBox> {
        let box_selector: SimpleBoxSelector = SimpleBoxSelector::new();
        if tokens.is_none() {
            let selected_boxes: BoxSelection<ErgoBox> = box_selector.select(input_boxes, nano_erg_amount, &[]).unwrap();
            return selected_boxes;
        }
        let tokens: &Vec<Token> = tokens.as_ref().unwrap();
        let selected_boxes: BoxSelection<ErgoBox> = box_selector.select(input_boxes, nano_erg_amount, &tokens).unwrap();
        return selected_boxes;
    }

    fn create_output_candidates(&mut self) -> Vec<ErgoBoxCandidate> {
        let height: u32 = get_current_height() as u32;

        let mut output_candidates: Vec<ErgoBoxCandidate> = Vec::new();

        let first_box_value: BoxValue = BoxValue::new(self.value).unwrap();
        let first_box_address: ErgoTree = convert_address_to_ergo_tree(&self.reciever);
        let mut first_box_builder: ErgoBoxCandidateBuilder = ErgoBoxCandidateBuilder::new(first_box_value, first_box_address, height);
        if self.tokens.is_some() {
            let tokens: &Vec<Token> = self.tokens.as_ref().unwrap();
            for t in tokens {
                first_box_builder.add_token(t.clone());
            }
        }
        let first_box: ErgoBoxCandidate = first_box_builder.build().unwrap();
        output_candidates.push(first_box);

        if self.alt_recievers.is_some() {
            let alt_recievers = self.alt_recievers.as_ref().unwrap();
            for r in alt_recievers {
                let output_value: BoxValue = BoxValue::new(r.value).unwrap();
                let output_address: ErgoTree = convert_address_to_ergo_tree(r.address.as_str());
                let mut output_box_builder: ErgoBoxCandidateBuilder = ErgoBoxCandidateBuilder::new(output_value, output_address, height);
                if r.tokens.is_some() {
                    let tokens: &Vec<Token> = r.tokens.as_ref().unwrap();
                    for t in tokens {
                        output_box_builder.add_token(t.clone());
                    }
                }
                output_candidates.push(output_box_builder.build().unwrap());
            }
        }

        return output_candidates;
    }

    pub fn get_signed_transaction_as_json(&mut self) -> String {
        let transaction_json: String = serde_json::to_string(&self.signed).unwrap();
        return transaction_json
    }

}

fn create_preheader(header: &Header) -> PreHeader {
    let preheader_version: u8 = header.version;
    let preheader_height: u32 = header.height;
    let preheader_timestamp: u64 = header.timestamp;
    let preheader_parent_id: &BlockId = &header.parent_id;
    let preheader_nbits: u64 = header.n_bits;
    let preheader_miner_pk: &Box<EcPoint> = &header.autolykos_solution.miner_pk;
    let preheader_votes: &Votes = &header.votes;
    let preheader: PreHeader = PreHeader { version: preheader_version, parent_id: preheader_parent_id.to_owned(), timestamp: preheader_timestamp, n_bits: preheader_nbits, height: preheader_height, miner_pk: preheader_miner_pk.to_owned(), votes: preheader_votes.to_owned() };
    return preheader;
}

pub fn convert_address_to_ergo_tree(address_str: &str) -> ErgoTree {
    let address: Address = convert_address_str_to_address(address_str);
    let ergo_tree: ErgoTree = address.script().unwrap();
    return ergo_tree;
}

pub fn convert_address_str_to_address(address_str: &str) -> Address {
    let address_encoder: AddressEncoder = AddressEncoder::new(NetworkPrefix::Mainnet);
    let address: Address = address_encoder.parse_address_from_str(address_str).unwrap();
    return address;
}