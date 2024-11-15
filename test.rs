// src/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet::TokenWallet;

    #[test]
    fn test_create_wallet() {
        let mut contract = TokenWalletContract::default();
        let wallet = contract.create_wallet("Alice".to_string(), "TokenA".to_string());
        assert_eq!(wallet.balance_of(), 0);
        assert_eq!(wallet.owner, "Alice");
    }

    #[test]
    fn test_send_and_receive_tokens() {
        let mut contract = TokenWalletContract::default();
        let alice_wallet = contract.create_wallet("Alice".to_string(), "TokenA".to_string());
        let bob_wallet = contract.create_wallet("Bob".to_string(), "TokenA".to_string());

        let mut alice_wallet = contract.get_wallet_by_owner("Alice").unwrap().clone();
        alice_wallet.receive_tokens(100);

        let mut bob_wallet = contract.get_wallet_by_owner("Bob").unwrap().clone();

        assert!(alice_wallet.send_tokens(50, &mut bob_wallet).is_ok());
        assert_eq!(alice_wallet.balance_of(), 50);
        assert_eq!(bob_wallet.balance_of(), 50);
    }

    #[test]
    fn test_insufficient_balance() {
        let mut contract = TokenWalletContract::default();
        let alice_wallet = contract.create_wallet("Alice".to_string(), "TokenA".to_string());

        let mut alice_wallet = contract.get_wallet_by_owner("Alice").unwrap().clone();
        let mut bob_wallet = contract.create_wallet("Bob".to_string(), "TokenA".to_string());

        assert_eq!(alice_wallet.send_tokens(50, &mut bob_wallet), Err("Insufficient balance".to_string()));
    }
}
