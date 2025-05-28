

# SocialLoginWallet

A modular, research-driven project exploring secure, decentralized wallet and key management for blockchain applications. The project implements and documents advanced cryptographic techniques such as Multi-Party Computation (MPC), Shamir’s Secret Sharing, Account Abstraction, and Social Authentication, with a focus on usability, security, and flexibility.

## Project Structure

- **SSSharing/**: Implements Shamir’s Secret Sharing for BIP-39 mnemonics, allowing secure splitting and recovery of wallet seeds. Each share is a valid mnemonic, supporting decoy wallets and flexible recovery.
- **MPC/**: Demonstrates Multi-Party Computation for private key management, ensuring no single party holds the full key. Useful for decentralized wallets and threshold cryptography.
- **AccountAbstraction/**: Explores account abstraction concepts, enabling programmable transaction logic, social recovery, multisig, and gas abstraction. Integrates secret sharing for enhanced security.
- **SocialAuth/**: Proposes stateless authentication using encryption and hashing, suitable for verifying Shamir shares or credentials without persistent storage.
- **Blockchain/**: Contains notes and diagrams on cryptographic standards (BIP, ECDSA, ECC) and wallet address derivation.
- **Backup/**: Rust code for wallet backup and recovery mechanisms.
- **Doc/**: Documentation on MPC, KMS (Key Management Service), and their applications in blockchain and Web3.

## Key Features

- **Shamir’s Secret Sharing for Wallets**: Securely split and recover wallet seeds using threshold cryptography.
- **MPC for Key Management**: Decentralized private key management, reducing single points of failure.
- **Account Abstraction**: Flexible account models supporting social recovery, multisig, and programmable logic.
- **Stateless Social Authentication**: Secure, token-based authentication without storing sensitive data.
- **Comprehensive Documentation**: In-depth notes and guides on cryptography, blockchain standards, and security best practices.

## Usage

Each module is self-contained and documented. See the respective `ReadMe.md` or documentation files in each directory for setup, usage, and examples.

### Example: Splitting a BIP-39 Mnemonic

```bash
cd SSSharing
cargo run --features double-check split -t 2 -n 3 --secret "your mnemonic here"
```

### Example: Running MPC Demo

```bash
cd MPC
cargo run
```

## Security Notice

This project is for research and educational purposes. The implementations are not audited and should not be used in production environments without thorough review.

## License

This project is licensed under the Apache 2.0 License.


