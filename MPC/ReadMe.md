# Multi-Party Computation (MPC) with Shamir's Secret Sharing in Rust

This project implements **Multi-Party Computation (MPC)** using **Shamir's Secret Sharing** scheme in Rust. The primary use case demonstrated is **Private Key Management**, where a private key is split into multiple shares, ensuring that no single party holds the entire key. A subset of these shares (the threshold) is required to reconstruct the key.

## Table of Contents

1. [Overview](#overview)
2. [How It Works](#how-it-works)
3. [Use Case: Private Key Management](#use-case-private-key-management)
4. [Installation](#installation)
5. [Usage](#usage)
6. [Security Considerations](#security-considerations)
7. [Contributing](#contributing)
8. [License](#license)

## Overview

### What is MPC?
**Multi-Party Computation (MPC)** is a cryptographic protocol that allows multiple participants to collaboratively compute a function while keeping their individual inputs private. In the context of private key management, MPC enables decentralized key management, preventing any single entity from holding the complete key, thus reducing the risk of compromise.

### Shamir's Secret Sharing
Shamir's Secret Sharing is a well-known algorithm for splitting a secret (like a private key) into multiple shares, where a subset (threshold) of shares is required to reconstruct the secret. This is useful in scenarios such as blockchain wallets, secure voting systems, and more.

## How It Works

- **Secret Splitting**: The private key (secret) is represented as a constant in a random polynomial. Each share corresponds to the value of the polynomial at a different point.
- **Reconstruction**: The secret can be reconstructed using **Lagrange interpolation** if enough shares (at least the threshold) are provided.
- **Threshold Cryptography**: A threshold (e.g., 3 out of 5 shares) is required to reconstruct the private key.

## Use Case: Private Key Management

In the context of blockchain or cryptocurrency wallets, MPC ensures secure private key management:
- The private key is split into multiple shares.
- No single party holds the full private key.
- Only a subset (threshold) of shares is required to perform operations such as signing transactions.
- This approach decentralizes key management, reducing the risk of theft or compromise.

## Installation

To install and run the project locally, follow these steps:

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (Ensure you have the latest version installed).

### Steps
1. Clone this repository:

   ```bash
   git clone https://github.com/your-username/mpc-key-management.git
   cd mpc-key-management

2. Add the necessary dependencies by modifying your Cargo.toml

    [dependencies]
    secrecy = "0.8"
    rand = "0.8"
    num-bigint = "0.4"
    num = "0.4"


3. Build and run the project:

    cargo build
    cargo run
