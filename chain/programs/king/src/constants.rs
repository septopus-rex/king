use anchor_lang::prelude::*;
use serde_json::{json, Value};

pub const SOLANA_PDA_LEN:usize=8;

//seeds 
pub const KING_COUNTER:&[u8;7]=b"counter";

#[account]
#[derive(InitSpace)]
pub struct KingIndex {
    pub value: u32,
}
impl KingIndex {
    pub fn inc(&mut self, amount:u32) {
        self.value += amount
    }

    ///!important, only on Devnet
    //FIXME, DEBUG only, need to remove when deploy on mainnet
    pub fn set(&mut self, amount:u32) {
        self.value = amount
    }
}
