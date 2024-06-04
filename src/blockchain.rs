use std::fmt::Write;

use serde_derive::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use chrono::prelude::*;

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct BlockHeader {
    nonce: i32,
    time_stamp: i32,
    prev_hash: String,
    merkle: String,
    difficulty: i32,
}

#[derive(Debug, Serialize)]
pub struct Block {
    header: BlockHeader,
    count: i32,
    transaction: Vec<Transaction>,
}

pub struct Chain {
    curr_tansc: Vec<Transaction>,
    chain: Vec<Block>,
    difficulty: i32,
    miner_address: String,
    reward: i32,
}

impl Chain {
    pub fn new(miner_address: String, difficulty: i32) -> Chain {
        let mut chain = Chain {
            curr_tansc: Vec::new(),
            chain: Vec::new(),
            difficulty,
            miner_address,
            reward: 10,
        };
        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: i32) -> bool {
        self.curr_tansc.push(Transaction {
            sender,
            receiver,
            amount,
        });

        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: i32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: i32) -> bool {
        self.reward = reward;
        true
    }

    pub fn generate_new_block(&mut self) -> bool {
        let mut header = BlockHeader {
            time_stamp: Utc::now().timestamp_millis() as i32,
            nonce: 0,
            prev_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };

        let reward_trans = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_address.clone(),
            amount: self.reward,
        };

        let mut block = Block {
            header: header.clone(),
            count: 0,
            transaction: Vec::new(),
        };

        block.transaction.push(reward_trans);
        block.transaction.append(&mut self.curr_tansc);
        block.count = block.transaction.len() as i32;
        block.header.merkle = Chain::get_merkle(block.transaction.clone());
        Chain::proof_of_work(&mut header);

        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

    fn get_merkle(curr_trans: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for t in &curr_trans {
            let hash = Chain::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }
        merkle.pop().unwrap()
    }

    pub fn proof_of_work(header: &mut BlockHeader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                }
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            };
        }
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let res = hasher.finalize();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();
        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("unable to write");
        }
        s
    }
}
