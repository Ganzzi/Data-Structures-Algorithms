use bincode;
use chrono::Utc;
use serde::Serialize;
use sha2::Digest;
use std::{thread, time::Duration};

use crate::base58::encode;

pub fn serialize<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: serde::Serialize,
{
    bincode::serialize(value).unwrap()
}

pub fn hash(input: &[u8]) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

#[derive(Serialize, Debug)]
pub struct BlockHeader {
    pre_hash: String,
    time: i64,
    tx_hash: String,
}

#[derive(Debug)]
pub struct Block {
    header: BlockHeader,
    txs: String,
    hash: String,
}

impl Block {
    pub fn new(tx: String, pre_hash: String) -> Self {
        println!("Start mining block...");
        thread::sleep(Duration::from_secs(2));

        let time = Utc::now().timestamp_millis();

        let mut block = Block {
            header: BlockHeader {
                pre_hash,
                time,
                tx_hash: hash(&serialize(&tx.clone())),
            },
            txs: tx,
            hash: String::new(),
        };

        block.set_hash();

        println!("Block mined: {}\n", &block.hash);

        block
    }

    fn set_hash(&mut self) {
        self.hash = hash(&serialize(&self.header));
    }
}

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![Blockchain::genesis_block()],
        }
    }

    fn genesis_block() -> Block {
        Block::new(
            "Mint 1m token to the creator!".to_string(),
            encode("initial hash"),
        )
    }

    pub fn add_block(&mut self, tx: String) {
        let pre_hash = self.blocks.last().unwrap().hash.clone();
        let block = Block::new(tx, pre_hash);
        self.blocks.push(block);
    }

    pub fn print_blockchain_info(&self) {
        for block in &self.blocks {
            println!("{:?}", block);
        }
    }
}
