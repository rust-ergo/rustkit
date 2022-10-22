use std::{collections::HashMap, thread::Scope, ptr::null};

use anyhow::Result;
use ergo_chain_types::{Header, PreHeader, BlockId, EcPoint, Votes};
use ergo_lib::{ergotree_ir::chain::ergo_box::ErgoBox, wallet::{box_selector::{SimpleBoxSelector, BoxSelection, BoxSelector, self}, tx_builder::TxBuilder, signing::TransactionContext, multi_sig::TransactionHintsBag}, chain::{ergo_box::box_builder::ErgoBoxCandidateBuilder, transaction::{unsigned::UnsignedTransaction, Transaction}, ergo_state_context::ErgoStateContext}};
use ergotree_ir::{chain::{ergo_box::{box_value::BoxValue, ErgoBoxCandidate, NonMandatoryRegisters, NonMandatoryRegisterId}, token::{Token, TokenAmount, TokenId}, address::{Address}}, ergo_tree::ErgoTree, mir::constant::Constant};

use wallet::wallet::RustKitWallet;

use crate::{address::create::{convert_address_str_to_address, convert_address_to_ergo_tree}, utils::consts::SUGGESTED_TX_FEE, config::file::Config};
use crate::wallet;

#[derive(Clone)]
pub struct RustKitOutputCandidate {
    receivers: Vec<Receiver>,
    pub registers: Option<Vec<NonMandatoryRegisters>>,
}

impl RustKitOutputCandidate {
    pub fn new(receivers: Vec<Receiver>) -> Self {
        RustKitOutputCandidate {
            receivers,
            registers: None,
        }
    }

    pub fn add_register(&mut self, register_type: &str, register_number: u8, register_value: &str) {
        if self.registers.is_none() {
            self.registers = Some(Vec::new());
        }

        let reg: NonMandatoryRegisters = match register_type {
            "SColl" => {
                let reg: NonMandatoryRegisters = SColl::new(register_number, register_value);
                reg
            }
            _ => {
                panic!("Register type not supported");
            }
        };
        self.registers.as_mut().unwrap().push(reg);
    }
}

pub struct RustKitUnsignedTransactionBuilder {
    inputs: Vec<ErgoBox>,
    data_inputs: Vec<ErgoBox>,
    outputs: Vec<RustKitOutputCandidate>,
    fee: u64,
    change_address: String,
    pub unsigned_tx: Option<UnsignedTransaction>,
}

impl RustKitUnsignedTransactionBuilder {
    pub fn new() -> Self {
        RustKitUnsignedTransactionBuilder {
            inputs: Vec::new(),
            data_inputs: Vec::new(),
            outputs: Vec::new(),
            fee: 0,
            change_address: String::new(),
            unsigned_tx: None,
        }
    }

    pub fn inputs(&mut self, inputs: Vec<ErgoBox>) {
        self.inputs = inputs;
    }

    pub fn data_inputs(&mut self) {
        todo!();
    }

    pub fn outputs(&mut self, outputs: Vec<RustKitOutputCandidate>) {
        self.outputs = outputs;
    }

    pub fn set_custom_fee(&mut self, fee: u64) {
        self.fee = fee;
    }

    pub fn set_min_fee(&mut self) {
        self.fee = SUGGESTED_TX_FEE;
    }

    pub fn set_change_address(&mut self, change_address: &str) {
        self.change_address = change_address.to_string();
    }

    pub fn build(&mut self) {
        let height: u32 = 0;

        let box_selector: SimpleBoxSelector = SimpleBoxSelector::new();
        let mut tokens_to_send: Vec<Token> = Vec::new();
        for opt in &self.outputs {
            for receiver in &opt.receivers {
                if receiver.tokens.is_some() {
                    let token_vec = receiver.tokens.as_ref().unwrap();
                    for token in token_vec {
                        let token = Token {
                            token_id: token.clone().token_id,
                            amount: TokenAmount::try_from(token.amount).unwrap(),
                        };
                        tokens_to_send.push(token);
                    }
                }
            }
        }

        let mut send_value: u64 = self.outputs.iter().fold(0, |acc, output| {
            acc + output.receivers.iter().fold(0, |acc, receiver| {
                acc + receiver.value
            })
        });
        send_value += self.fee;
        let mut selected_boxes: BoxSelection<ErgoBox> = box_selector.select(self.inputs.clone(), BoxValue::new(send_value).unwrap(), &[]).unwrap();
        if tokens_to_send.len() > 0 {
            let mut tokens: Vec<Token> = Vec::new();
            for tk in tokens_to_send.iter() {
                let tok = Token {
                    token_id: tk.token_id.clone(),
                    amount: TokenAmount::try_from(tk.amount).unwrap(),
                };
                tokens.push(tok);
                }
            selected_boxes = box_selector.select(self.inputs.clone(), BoxValue::new(send_value).unwrap(), &tokens).unwrap();
        }
        let output_candidates: Vec<ErgoBoxCandidate> = Self::convert_outputs(self.outputs.clone(), height);
        let change_address: Address = convert_address_str_to_address(&self.change_address);
        let transaction_builder: TxBuilder<ErgoBox> = TxBuilder::new(selected_boxes, output_candidates, height, BoxValue::new(self.fee).unwrap(), change_address);
        let unsigned_transaction: UnsignedTransaction = transaction_builder.build().unwrap();
        self.unsigned_tx = Some(unsigned_transaction);
    }

    fn convert_outputs(opt: Vec<RustKitOutputCandidate>, height: u32) -> Vec<ErgoBoxCandidate> {
        let mut output_candidates: Vec<ErgoBoxCandidate> = Vec::new();
        for o in opt {
            for rec in o.receivers {
                let box_value: BoxValue = BoxValue::new(rec.value).unwrap();
                let box_address: ErgoTree = convert_address_to_ergo_tree(&rec.address);
                let mut box_builder: ErgoBoxCandidateBuilder = ErgoBoxCandidateBuilder::new(box_value, box_address, height);
                if rec.tokens.is_some() {
                    let token_data: Vec<TokenData> = rec.tokens.unwrap();
                    for t in token_data {
                        let token_id: TokenId = t.token_id;
                        let token_amount: TokenAmount = TokenAmount::try_from(t.amount).unwrap();
                        let token: Token = Token { token_id, amount: token_amount };
                        box_builder.add_token(token);
                    }
                }
                let box_: ErgoBoxCandidate = box_builder.build().unwrap();
                output_candidates.push(box_);
            }
        }
        output_candidates
    }
}

#[derive(Clone)]
pub struct Receiver {
    address: String,
    value: u64,
    tokens: Option<Vec<TokenData>>,
}

impl Receiver {
    pub fn new(to: &str, value: u64) -> Self {
        Receiver {
            address: to.to_string(),
            value,
            tokens: None,
        }
    }

    pub fn add_token(&mut self, token_id: &str, amount: u64) {
        let token: TokenData = TokenData::new(token_id, amount);
        if self.tokens.is_none() {
            self.tokens = Some(Vec::new());
        }
        self.tokens.as_mut().unwrap().push(token);
    }
}

#[derive(Clone)]
pub struct TokenData {
    token_id: TokenId,
    amount: u64,
}

impl TokenData {
    pub fn new(token_id: &str, amount: u64) -> Self {
        let id_base16: Vec<u8> = base16::decode(token_id).unwrap();
        let id_base64: String = base64::encode(id_base16);
        let token_id: TokenId = TokenId::from_base64(&id_base64).unwrap();
        TokenData {
            token_id,
            amount,
        }
    }
}

pub struct SColl {
    pub register: NonMandatoryRegisters,
}

impl SColl {
    pub fn new(number: u8, value: &str) -> NonMandatoryRegisters {
        let value_base16: Vec<u8> = base16::decode(value).unwrap();

        let mut regsiter_id: NonMandatoryRegisterId = NonMandatoryRegisterId::R4;
        match number {
            4 => {
                regsiter_id = NonMandatoryRegisterId::R4;
            }
            5 => {
                regsiter_id = NonMandatoryRegisterId::R5;
            }
            6 => {
                regsiter_id = NonMandatoryRegisterId::R6;
            }
            7 => {
                regsiter_id = NonMandatoryRegisterId::R7;
            }
            8 => {
                regsiter_id = NonMandatoryRegisterId::R8;
            }
            9 => {
                regsiter_id = NonMandatoryRegisterId::R9;
            }
            _ => {
                panic!("Invalid register number");
            }
        }
        let constant: Constant = Constant::try_from(value_base16).unwrap();
        let mut registers: HashMap<NonMandatoryRegisterId, Constant> = HashMap::new();
        registers.insert(regsiter_id, constant);
        let reg: NonMandatoryRegisters = NonMandatoryRegisters::new(registers).unwrap();
        reg
    }
}