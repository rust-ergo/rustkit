use std::{collections::HashMap, thread::Scope};

use anyhow::Result;
use ergo_chain_types::{Header, PreHeader, BlockId, EcPoint, Votes};
use ergo_lib::{ergotree_ir::chain::ergo_box::ErgoBox, wallet::{box_selector::{SimpleBoxSelector, BoxSelection, BoxSelector}, tx_builder::TxBuilder, signing::TransactionContext, multi_sig::TransactionHintsBag}, chain::{ergo_box::box_builder::ErgoBoxCandidateBuilder, transaction::{unsigned::UnsignedTransaction, Transaction}, ergo_state_context::ErgoStateContext}};
use ergotree_ir::{chain::{ergo_box::{box_value::BoxValue, ErgoBoxCandidate, NonMandatoryRegisters, NonMandatoryRegisterId}, token::{Token, TokenAmount, TokenId}, address::{Address}}, ergo_tree::ErgoTree, mir::constant::Constant};

use wallet::wallet::RustKitWallet;

use crate::{address::create::{convert_address_str_to_address, convert_address_to_ergo_tree}, utils::consts::SUGGESTED_TX_FEE, config::file::Config};
use crate::wallet;

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

pub struct RustKitUnsignedTransaction {
    inputs: Vec<ErgoBox>,
    data_inputs: Vec<ErgoBox>,
    outputs: Vec<RustKitOutputCandidate>,
    fee: u64,
    change_address: String,
}

impl RustKitUnsignedTransaction {
    pub fn new() -> Self {
        RustKitUnsignedTransaction {
            inputs: Vec::new(),
            data_inputs: Vec::new(),
            outputs: Vec::new(),
            fee: 0,
            change_address: String::new(),
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
        todo!();
    }
}

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
        let constant = Constant::try_from(value_base16).unwrap();
        let mut registers: HashMap<NonMandatoryRegisterId, Constant> = HashMap::new();
        registers.insert(regsiter_id, constant);
        let reg = NonMandatoryRegisters::new(registers).unwrap();
        reg
    }
}