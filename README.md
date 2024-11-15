This project implements a basic secure token wallet on the Internet Computer Protocol (ICP) blockchain using Rust. The wallet supports fundamental functionalities, such as:

Sending and receiving tokens
Displaying the current token balance
Basic wallet security checks
The purpose of this project is to demonstrate Rust programming proficiency, specifically in the context of smart contract development on the ICP blockchain.
Features:
Token Transfer: Users can transfer tokens to other wallet addresses.
Receive Tokens: The wallet can receive tokens from other addresses.
Balance Display: Displays the current balance of tokens in the wallet.
Security: Basic security checks to ensure only authorized users can execute certain actions.
Ensure you have the following installed:

Rust: Install Rust by running:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
DFINITY SDK (for ICP development): Install by running:
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
Verify the installation with:

dfx --version:"
repository is organised as :
- src/
  - lib.rs              # Main library for smart contract logic
  - wallet.rs           # Token wallet implementation with send, receive, and balance functions
  - tests.rs            # Unit tests for validating wallet functions
- Cargo.toml            # Rust package configuration
INSTALLATION :
Clone the Repository:
git clone https://github.com/yourusername/icp-token-wallet.git
cd icp-token-wallet
Install Dependencies: Install any required dependencies listed in Cargo.toml by running:
cargo build
Building and Deploying
Start the Local ICP Network: The DFINITY SDK includes a local ICP network for development. Run it in the background:

dfx start --background
Deploy the Smart Contract: Deploy the contract to the local network:

dfx deploy
Running Tests
To validate the functions and security of the token wallet, use Rust's built-in testing framework:

cargo test
The tests.rs file includes several unit tests that:

Validate wallet creation
Test token sending and receiving functions
Check for errors when attempting transactions with insufficient balance
Usage
Once deployed, you can interact with the token wallet through the following functions:

Create Wallet: Initializes a new wallet with an owner and a specific token ID.

let mut contract = TokenWalletContract::default();
let wallet = contract.create_wallet("Alice".to_string(), "TokenA".to_string());
Check Balance: Returns the current token balance of a specified wallet.

let balance = wallet.balance_of();
println!("Current balance: {}", balance);
Send Tokens: Transfers tokens from one wallet to another. Ensure the sender has a sufficient balance.

let mut sender_wallet = contract.get_wallet_by_owner("Alice").unwrap();
let mut receiver_wallet = contract.get_wallet_by_owner("Bob").unwrap();
sender_wallet.send_tokens(50, &mut receiver_wallet).unwrap();
Receive Tokens: Receives tokens into a wallet, increasing its balance.

receiver_wallet.receive_tokens(100);
CODE EXPLAINATION:
TokenWallet Struct
This struct defines the structure and core functionalities of the token wallet, including fields for balance, owner, and token_id.

pub struct TokenWallet {
    pub balance: u64,
    pub owner: String,
    pub token_id: String,
}
Core Functions
send_tokens: Checks if the sender has a sufficient balance and performs the transfer if valid.
receive_tokens: Increases the wallet balance by the received token amount.
balance_of: Returns the current token balance.
Contract Functions
create_wallet: Initializes and adds a new wallet to the contract.
get_wallet_by_owner: Searches for and retrieves a wallet by its owner.
transfer_tokens: Facilitates token transfer between two wallets using the owners' names.
TokenWallet Struct
This struct defines the structure and core functionalities of the token wallet, including fields for balance, owner, and token_id.

pub struct TokenWallet {
    pub balance: u64,
    pub owner: String,
    pub token_id: String,
}
Core Functions
send_tokens: Checks if the sender has a sufficient balance and performs the transfer if valid.
receive_tokens: Increases the wallet balance by the received token amount.
balance_of: Returns the current token balance.
Contract Functions
create_wallet: Initializes and adds a new wallet to the contract.
get_wallet_by_owner: Searches for and retrieves a wallet by its owner.
transfer_tokens: Facilitates token transfer between two wallets using the owners' names.
Example Transactions
Here are some example transactions you can run in Rust code after setting up the project:

fn main() {
    // Initialize the contract and create wallets for Alice and Bob
    let mut contract = TokenWalletContract::default();
    let alice_wallet = contract.create_wallet("Alice".to_string(), "TokenA".to_string());
    let bob_wallet = contract.create_wallet("Bob".to_string(), "TokenA".to_string());

    // Deposit tokens into Alice's wallet
    let mut alice_wallet = contract.get_wallet_by_owner("Alice").unwrap().clone();
    alice_wallet.receive_tokens(100);
    println!("Alice's initial balance: {}", alice_wallet.balance_of());

    // Transfer tokens from Alice to Bob
    let mut bob_wallet = contract.get_wallet_by_owner("Bob").unwrap().clone();
    match alice_wallet.send_tokens(50, &mut bob_wallet) {
        Ok(_) => println!("Transfer successful!"),
        Err(e) => println!("Transfer failed: {}", e),
    }

    println!("Alice's balance after transfer: {}", alice_wallet.balance_of());
    println!("Bob's balance after receiving tokens: {}", bob_wallet.balance_of());
}
