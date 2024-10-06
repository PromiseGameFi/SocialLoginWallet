use secp256k1::{SecretKey, PublicKey, Message, Secp256k1};
use threshold_crypto::{SecretKey as ThresholdSecretKey, SecretKeySet, SecretKeyShare};
use ethereum_types::{H160, U256};
use std::collections::HashMap;

struct MPCWallet {
    threshold: usize,
    secret_shares: Vec<SecretKeyShare>,
    public_key: PublicKey,
    token_balances: HashMap<H160, U256>, // ERC20 token address -> balance
}

impl MPCWallet {
    fn new(threshold: usize, total_shares: usize) -> Self {
        let secret_key_set = SecretKeySet::random(threshold, &mut rand::thread_rng());
        let public_key = secret_key_set.public_keys().public_key();
        let secret_shares = (0..total_shares)
            .map(|i| secret_key_set.secret_key_share(i))
            .collect();

        let secp = Secp256k1::new();
        let private_key = SecretKey::from_slice(secret_key_set.secret_key().to_bytes().as_slice()).unwrap();
        let public_key = PublicKey::from_secret_key(&secp, &private_key);

        MPCWallet {
            threshold,
            secret_shares,
            public_key,
            token_balances: HashMap::new(),
        }
    }

    fn store_tokens(&mut self, token_address: H160, amount: U256) {
        let balance = self.token_balances.entry(token_address).or_insert(U256::zero());
        *balance += amount;
    }

    fn get_token_balance(&self, token_address: H160) -> U256 {
        *self.token_balances.get(&token_address).unwrap_or(&U256::zero())
    }

    fn sign_message(&self, message: &str, shares: Vec<SecretKeyShare>) -> Option<secp256k1::Signature> {
        if shares.len() < self.threshold {
            return None; // Not enough shares to reconstruct
        }

        let reconstructed_key = ThresholdSecretKey::combine_shares(shares.iter().take(self.threshold)).unwrap();
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(reconstructed_key.to_bytes().as_slice()).unwrap();
        let message = Message::from_slice(message.as_bytes()).unwrap();

        Some(secp.sign(&message, &secret_key))
    }

    fn transfer_tokens(&mut self, token_address: H160, to: H160, amount: U256) -> bool {
        let balance = self.token_balances.entry(token_address).or_insert(U256::zero());
        if *balance >= amount {
            *balance -= amount;
            println!("Transferred {} tokens to {}", amount, to);
            return true;
        } else {
            println!("Insufficient balance");
            return false;
        }
    }
}

fn main() {
    let mut wallet = MPCWallet::new(3, 5);

    let token_address: H160 = "0000000000000000000000000000000000000001".parse().unwrap();
    wallet.store_tokens(token_address, U256::from(1000));
    let balance = wallet.get_token_balance(token_address);
    println!("Token balance: {}", balance);

    let shares = wallet.secret_shares.clone(); // Simulate having all shares
    let message = "Transaction to sign";
    if let Some(signature) = wallet.sign_message(message, shares) {
        println!("Message signed successfully: {:?}", signature);
    } else {
        println!("Not enough shares to sign the message");
    }

    let recipient: H160 = "0000000000000000000000000000000000000002".parse().unwrap();
    wallet.transfer_tokens(token_address, recipient, U256::from(500));
}
