# Wallet Seed Phrase Generation

This document outlines the technology and processes used in generating and managing seed phrases for cryptocurrency wallets.

## Overview

Wallets use various technologies to generate secure and unique seed phrases, which are crucial for creating and restoring cryptocurrency wallets.

## Key Components

### 1. Entropy Sources
- Wallets utilize random number generators (RNGs) to collect entropy from diverse sources, such as:
  - Mouse movements
  - Keyboard input
  - Hardware-based random number generators

### 2. Mnemonic Code Generation
- The seed phrase is generated using the **BIP39** standard, which creates a human-readable mnemonic phrase from the random seed. This typically consists of:
  - 12 or 24 words.

### 3. Hashing Algorithms
- After the seed is generated, wallets often use hashing algorithms (e.g., **SHA-256**) to derive private keys from the seed phrase, ensuring the security and integrity of the keys.

### 4. Key Derivation Functions
- Standards like **BIP32** or **BIP44** are employed for hierarchical deterministic wallets, allowing users to derive multiple addresses from a single seed phrase.

### 5. Secure Storage
- Wallets must securely store the seed phrase, commonly using encryption methods to protect against unauthorized access.

## Conclusion

These technologies work together to ensure that seed phrases are both secure and user-friendly, enabling users to recover their wallets as needed.
