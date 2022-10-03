use anyhow::Result;
use ergo_chain_types::{Header, PreHeader, BlockId, EcPoint, Votes};
use ergo_lib::{ergotree_ir::chain::ergo_box::ErgoBox, wallet::{box_selector::{SimpleBoxSelector, BoxSelection, BoxSelector}, tx_builder::TxBuilder, signing::TransactionContext, multi_sig::TransactionHintsBag}, chain::{ergo_box::box_builder::ErgoBoxCandidateBuilder, transaction::{unsigned::UnsignedTransaction, Transaction}, ergo_state_context::ErgoStateContext}};
use ergotree_ir::{chain::{ergo_box::{box_value::BoxValue, ErgoBoxCandidate}, token::{Token, TokenAmount, TokenId}, address::{Address}}, ergo_tree::ErgoTree};

use wallet::wallet::RustKitWallet;

use crate::{address::create::{convert_address_str_to_address, convert_address_to_ergo_tree}, utils::consts::SUGGESTED_TX_FEE, config::file::Config};
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

struct Mint {
    name: String,
    description: String,
    amount: u64,
    decimals: usize,
}

impl Mint {

    pub fn new(name: String, description: String, amount: u64, decimals: usize) -> Self {
        Mint {
            name,
            description,
            amount,
            decimals,
        }
    }
}

/// RustKit implementation of transaction
pub struct RustKitTransaction { 
    reciever: String,
    alt_recievers: Option<Vec<Reciver>>,
    value: u64,
    fee: u64,
    send_tokens: Option<Vec<Token>>,
    mint_tokens: Option<Mint>,
    input_boxes: Option<Vec<ErgoBox>>,
    data_boxes: Option<Vec<ErgoBox>>,
    unsigned: Option<UnsignedTransaction>,
    signed: Option<Transaction>,
    wallet: RustKitWallet,
    config: Config,
}

impl RustKitTransaction {
    /// Create a new transaction
    pub fn new(receiver_address: &str, amount_to_send: u64, wallet: RustKitWallet, config: Config) -> Self {
        RustKitTransaction {
            reciever: receiver_address.to_owned(),
            alt_recievers: None,
            value: amount_to_send,
            fee: SUGGESTED_TX_FEE,
            send_tokens: None,
            mint_tokens: None,
            input_boxes: None,
            data_boxes: None,
            unsigned: None,
            signed: None,
            wallet: wallet,
            config: config,
        }
    }

    /// Build the transaction. Creates an unsigned transaction.
    pub fn build(&mut self) {
        let height: u32 = ergo_rustkit_endpoints::get_current_height(self.config.explorer_url.clone()) as u32;
        let input_boxes_raw: Option<Vec<ErgoBox>> = self.wallet.get_input_boxes();
        if input_boxes_raw.is_none() {
            panic!("No input boxes found for address: {}", self.wallet.index_0_address);
        }
        let input_boxes_explorer: Vec<ErgoBox> = input_boxes_raw.unwrap();
        self.input_boxes = Some(input_boxes_explorer);

        let tx_input_boxes: BoxSelection<ErgoBox> = Self::get_input_boxes(self);
        
        let mut selected_boxes: Vec<ErgoBox> = Vec::new();
        for b in tx_input_boxes.clone().boxes {
            selected_boxes.push(b);
        }
        self.input_boxes = Some(selected_boxes);

        let tx_output_boxes: Vec<ErgoBoxCandidate> = Self::create_output_candidates(self);

        let fee_amount: BoxValue = BoxValue::new(self.fee).unwrap();

        let change_address: Address = convert_address_str_to_address(self.wallet.index_0_address.as_str());

        let data_boxes: Vec<ErgoBox> = Vec::new();
        self.data_boxes = Some(data_boxes);

        let transaction_builder: TxBuilder<ErgoBox> = TxBuilder::new(tx_input_boxes, tx_output_boxes, height, fee_amount, change_address);
        let unsigned: UnsignedTransaction = transaction_builder.build().unwrap();
        self.unsigned = Some(unsigned);
    }

    /// Signs the unsigned transaction
    pub fn sign(&mut self) {
        let last_10_headers: [Header; 10] = ergo_rustkit_endpoints::get_last_10_headers(self.config.explorer_url.clone(), self.config.node_url.clone());
        let preheader: PreHeader = create_preheader(&last_10_headers[0]);
        let transaction_context: TransactionContext<UnsignedTransaction> = TransactionContext::new(self.unsigned.clone().unwrap(), self.input_boxes.clone().unwrap(), self.data_boxes.clone().unwrap()).unwrap();
        let state_context: ErgoStateContext = ErgoStateContext::new(preheader, last_10_headers);
        let transaction_hints: TransactionHintsBag = self.wallet.wallet.generate_commitments(transaction_context.clone(), &state_context).unwrap();
        let signed_transaction: Transaction = self.wallet.wallet.sign_transaction(transaction_context, &state_context, Some(&transaction_hints)).unwrap();
        self.signed = Some(signed_transaction);
    }

    /// Submits the signed transaction to the network
    pub fn submit(&mut self) -> Result<String> {
        let transaction_json: String = self.get_signed_transaction_as_json();
        let resp: Result<String> = ergo_rustkit_endpoints::submit(transaction_json, self.config.node_url.clone());
        resp
    }

    /// Add a reciever for multiple recievers
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

    /// Add a token to send. Will be sent to first reciever
    pub fn add_token(&mut self, token_id: &str, amount: u64) {
        let id_base16: Vec<u8> = base16::decode(token_id).unwrap();
        let id_base64: String = base64::encode(&id_base16);
        let tk = Token {
            token_id: TokenId::from_base64(&id_base64).unwrap(),
            amount: TokenAmount::try_from(amount as u64).unwrap(),
        };
        let token_vec: &Option<Vec<Token>> = &self.send_tokens;
        let token_vec: Option<Vec<Token>> = token_vec.to_owned();
        if token_vec.is_none() {
            let mut tokens: Vec<Token> = Vec::new();
            tokens.push(tk);
            self.send_tokens = Some(tokens);
        } else {
            let mut tokens: Vec<Token> = token_vec.unwrap();
            tokens.push(tk);
            self.send_tokens = Some(tokens);
        }
    }

    /// Mint a new token
    pub fn mint_token(&mut self, name: &str, description: &str, amount: u64, decimals: usize) {
        let mint: Mint = Mint::new(name.to_owned(), description.to_owned(), amount, decimals);
        self.mint_tokens = Some(mint);
    }

    /// Set the fee for the transaction
    pub fn set_fee(&mut self, fee: u64) {
        self.fee = fee;
    }

    fn get_input_boxes(&mut self) -> BoxSelection<ErgoBox> {
        let mut nano_erg_amount: BoxValue = BoxValue::try_from(self.value + self.fee).unwrap();

        if self.mint_tokens.is_some() {
            nano_erg_amount = BoxValue::try_from(self.value + self.fee + BoxValue::SAFE_USER_MIN.as_u64()).unwrap();
        }

        let box_selector: SimpleBoxSelector = SimpleBoxSelector::new();
        if self.send_tokens.is_none() {
            let selected_boxes: BoxSelection<ErgoBox> = box_selector.select(self.input_boxes.clone().unwrap(), nano_erg_amount, &[]).unwrap();
            return selected_boxes;
        }
        let tokens: &Vec<Token> = self.send_tokens.as_ref().unwrap();
        let selected_boxes: BoxSelection<ErgoBox> = box_selector.select(self.input_boxes.clone().unwrap(), nano_erg_amount, tokens).unwrap();
        selected_boxes
    }

    fn create_output_candidates(&mut self) -> Vec<ErgoBoxCandidate> {
        let height: u32 = ergo_rustkit_endpoints::get_current_height(self.config.explorer_url.clone()) as u32;

        let mut output_candidates: Vec<ErgoBoxCandidate> = Vec::new();

        let first_box_value: BoxValue = BoxValue::new(self.value).unwrap();
        let first_box_address: ErgoTree = convert_address_to_ergo_tree(&self.reciever);
        let mut first_box_builder: ErgoBoxCandidateBuilder = ErgoBoxCandidateBuilder::new(first_box_value, first_box_address, height);
        if self.send_tokens.is_some() {
            let tokens: &Vec<Token> = self.send_tokens.as_ref().unwrap();
            for t in tokens {
                first_box_builder.add_token(t.clone());
            }
        }
        let first_box: ErgoBoxCandidate = first_box_builder.build().unwrap();
        output_candidates.push(first_box);

        if self.mint_tokens.is_some() {
            let mint = self.mint_tokens.as_ref().unwrap();
            let mint_id = self.input_boxes.as_ref().unwrap()[0].box_id();
            let mint_box_value: BoxValue = BoxValue::SAFE_USER_MIN;
            let mint_box_address: ErgoTree = convert_address_to_ergo_tree(&self.reciever);
            let mut mint_box_builder: ErgoBoxCandidateBuilder = ErgoBoxCandidateBuilder::new(mint_box_value, mint_box_address, height);
            let mint_token = Token {
                token_id: TokenId::from(mint_id),
                amount: TokenAmount::try_from(mint.amount).unwrap(),
            };
            mint_box_builder.mint_token(mint_token, mint.name.clone(), mint.description.clone(), mint.decimals);
            let mint_box: ErgoBoxCandidate = mint_box_builder.build().unwrap();
            output_candidates.push(mint_box);
        }

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

        output_candidates
    }

    /// Returns the transaction as a json string
    pub fn get_signed_transaction_as_json(&mut self) -> String {
        let transaction_json: String = serde_json::to_string(&self.signed).unwrap();
        transaction_json
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
    preheader
}