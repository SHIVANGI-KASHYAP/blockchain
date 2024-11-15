// src/wallet.rs
#[derive(Clone)]
pub struct TokenWallet {
    pub balance: u64,
    pub owner: String,
    pub token_id: String,
}

impl TokenWallet {
    /// Creates a new token wallet with the specified owner and token_id
    pub fn new(owner: String, token_id: String) -> Self {
        TokenWallet {
            balance: 0,
            owner,
            token_id,
        }
    }

    /// Send tokens to another wallet
    pub fn send_tokens(&mut self, amount: u64, recipient: &mut TokenWallet) -> Result<(), String> {
        if self.balance >= amount {
            if self.owner == recipient.owner {
                return Err(String::from("Cannot send tokens to the same wallet"));
            }
            self.balance -= amount;
            recipient.receive_tokens(amount);
            Ok(())
        } else {
            Err(String::from("Insufficient balance"))
        }
    }

    /// Receive tokens in the wallet
    pub fn receive_tokens(&mut self, amount: u64) {
        self.balance += amount;
    }

    /// Display current balance of tokens
    pub fn balance_of(&self) -> u64 {
        self.balance
    }
}
