
use threshold_crypto::{PublicKeySet, SecretKeySet, SecretKeyShare, SignatureShare};
use ethereum_types::{Address, U256, H256};
use sha3::{Digest, Keccak256};
use rand::Rng;

struct AbstractAccount {
    address: Address,
    public_key_set: PublicKeySet,
    threshold: usize,
}

impl AbstractAccount {
    fn new(threshold: usize, total_shares: usize) -> Self {
        let mut rng = rand::thread_rng();
        let secret_key_set = SecretKeySet::random(threshold - 1, &mut rng);
        let public_key_set = secret_key_set.public_keys();
        
        // Derive Ethereum address from the group's public key
        let public_key_bytes = public_key_set.public_key().to_bytes();
        let address = Address::from_slice(&Keccak256::digest(&public_key_bytes)[12..]);
        
        AbstractAccount {
            address,
            public_key_set,
            threshold,
        }
    }


    fn verify_transaction(&self, transaction: &Transaction, signatures: &[SignatureShare]) -> bool {
        if signatures.len() < self.threshold {
            return false;
        }

        let message = self.hash_transaction(transaction);
        let combined_signature = self.public_key_set
            .combine_signatures(signatures)
            .expect("Failed to combine signatures");

        self.public_key_set.public_key().verify(&combined_signature, message)
    }

    fn hash_transaction(&self, transaction: &Transaction) -> H256 {
        let mut hasher = Keccak256::new();
        
        hasher.update(&transaction.value.to_be_bytes_vec());
        hasher.update(&transaction.nonce.to_be_bytes_vec());
        hasher.update(&transaction.data);
        H256::from_slice(&hasher.finalize())
    }
}

struct Transaction {
    to: Address,
    value: U256,
    nonce: U256,
    data: Vec<u8>,
}

fn main() {
    // Create an abstract account with a 2-of-3 threshold
    let account = AbstractAccount::new(2, 3);
    println!("Account address: {:?}", account.address);

    // Create a sample transaction
    let transaction = Transaction {
        to: Address::zero(),
        value: U256::from(1_000_000_000_000_000_000u64), // 1 ETH
        nonce: U256::zero(),
        data: vec![],
    };

   
}
