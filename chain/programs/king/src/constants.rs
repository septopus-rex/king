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
    
    pub fn set(&mut self, amount:u32) {
        self.value = amount
    }
}
