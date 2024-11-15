// src/lib.rs
mod wallet;

use wallet::TokenWallet;

#[derive(Default)]
pub struct TokenWalletContract {
    pub wallets: Vec<TokenWallet>,
}

impl TokenWalletContract {
    /// Initialize the smart contract with a new wallet
    pub fn create_wallet(&mut self, owner: String, token_id: String) -> &TokenWallet {
        let wallet = TokenWallet::new(owner.clone(), token_id);
        self.wallets.push(wallet);
        self.wallets.last().unwrap()
    }

    /// Get a wallet by owner
    pub fn get_wallet_by_owner(&self, owner: &str) -> Option<&TokenWallet> {
        self.wallets.iter().find(|wallet| wallet.owner == owner)
    }

    /// Send tokens between two wallets by owner names
    pub fn transfer_tokens(&mut self, sender_owner: &str, receiver_owner: &str, amount: u64) -> Result<(), String> {
        let sender = self.wallets.iter_mut().find(|wallet| wallet.owner == sender_owner);
        let receiver = self.wallets.iter_mut().find(|wallet| wallet.owner == receiver_owner);

        match (sender, receiver) {
            (Some(sender_wallet), Some(receiver_wallet)) => {
                sender_wallet.send_tokens(amount, receiver_wallet)
            }
            _ => Err(String::from("Wallet not found")),
        }
    }
}
