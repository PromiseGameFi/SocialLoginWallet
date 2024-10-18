use shamir_secret_sharing::ShamirSecretSharing as SSS;
use num_bigint::{BigInt, BigUint};
use num_bigint::Sign::*; // Ensure you are using what you need.

fn main() {
    let sss = SSS {
        threshold: 3,
        share_amount: 5,
        prime: BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).unwrap(),
    };

    let secret = BigInt::parse_bytes(b"ffffffffffffffffffffffffffffffffffffff", 16).unwrap();

    // Split the secret into shares
    let shares = sss.split(secret.clone());
    

    println!("shares: {:?}", shares);

    // Recover the secret using the threshold number of shares
    assert_eq!(secret, sss.recover(&shares[0..sss.threshold as usize]));
}
