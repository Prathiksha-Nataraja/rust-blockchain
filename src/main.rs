#[macro_use]
extern crate serde_derive;
mod tests;
mod blockchain;

use std::io;
use std::io::Write;
use std::process;

fn main() {
    println!("Hello, world!");
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choose = String::new();

    println!("Input a miner address");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);

    println!("Input the Difficulty");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);

    let diff = difficulty
        .trim()
        .parse::<i32>()
        .expect("we need an integer");
    println!("generating genesis block! ");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1. Transaction");
        println!("2. Mine the block");
        println!("3. Change Difficulty");
        println!("4. Change Reward");
        println!("5. Exit");
        print!("Enter your choise");
        io::stdout().flush();
        choose.clear();
        io::stdin().read_line(&mut choose);
        println!("");

        match choose.trim().parse().unwrap() {
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("enter sender address:");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);
                print!("enter receiver address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);
                print!("Enter amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("transaction added"),
                    false => println!("transaction failed"),
                }
            }
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                print!("enter new difficulty: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Updated Difficulty"),
                    false => println!("Failed Update Difficulty"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated reward"),
                    false => println!("Failed Update reward"),
                }
            }
            5 => {
                println!("exiting!");
                process::exit(0);
            }
            _ => println!("Invalid option please retry"),
        }
    }
}
