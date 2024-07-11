#[cfg(test)]
mod tests {
    use crate::blockchain::Chain;

    use super::*;

    #[test]
    fn test_new_transaction() {
        let mut chain = Chain::new(String::from("miner"), 2);
        assert_eq!(
            chain.new_transaction(String::from("Alice"), String::from("Bob"), 50),
            true
        );
        assert_eq!(chain.curr_tansc.len(), 1);
        assert_eq!(chain.curr_tansc[0].sender, "Alice");
        assert_eq!(chain.curr_tansc[0].receiver, "Bob");
        assert_eq!(chain.curr_tansc[0].amount, 50);
    }

    #[test]
    fn test_update_difficulty() {
        let mut chain = Chain::new(String::from("miner"), 2);
        assert_eq!(chain.update_difficulty(5), true);
        assert_eq!(chain.difficulty, 5);
    }

    #[test]
    fn test_generate_new_block() {
        let mut chain = Chain::new(String::from("miner"), 2);
        chain.new_transaction(String::from("Alice"), String::from("Bob"), 100);
        assert_eq!(chain.generate_new_block(), true);
        // Includes the initial block generated in `new`
        assert_eq!(chain.chain.len(), 2);
        // Includes reward transaction
        assert_eq!(chain.chain[1].transaction.len(), 2);
        assert_eq!(chain.chain[1].transaction[1].sender, "Alice");
        assert_eq!(chain.chain[1].transaction[1].receiver, "Bob");
        assert_eq!(chain.chain[1].transaction[1].amount, 100);
    }
}
