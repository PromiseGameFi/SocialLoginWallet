# Bitcoin Improvement Proposals (BIPs)

Bitcoin Improvement Proposals (BIPs) are formal documents that describe new features, improvements, or changes to the Bitcoin protocol. There are numerous BIPs, each focusing on specific aspects of the Bitcoin ecosystem. Below is a categorized list of some of the most well-known and important BIPs, including their numbers, titles, and purposes:

## 1. Wallet and Address Standards
These BIPs are related to wallet functionalities, including address formats and key derivation methods:

- **BIP-32**: Hierarchical Deterministic Wallets  
  Describes how to generate hierarchical deterministic (HD) wallets, allowing for a single seed to generate a tree of private and public keys.

- **BIP-39**: Mnemonic Code for Generating Deterministic Keys  
  Defines a method for creating a human-readable mnemonic seed phrase, which can be used to generate a seed for deterministic wallets.

- **BIP-44**: Multi-Account Hierarchy for Deterministic Wallets  
  Extends BIP-32 by specifying a multi-level structure for deterministic wallets, allowing the derivation of keys for multiple accounts, coins, and purposes.

- **BIP-49**: Derivation Scheme for P2WPKH-nested-in-P2SH based Accounts  
  Defines the derivation paths for SegWit addresses (nested in P2SH), extending BIP-44 for backward-compatible SegWit addresses.
  
  SegWit is the most common BTC address format that almost all businesses and wallets support. It starts with the number 3 and contains 26 to 36 characters. 
  One starts with '3' (P2SH format), which is more common, and the other starts with 'bc1' (bech32 format).

- **BIP-84**: Derivation Scheme for P2WPKH based Accounts  
  Defines the derivation paths for native SegWit addresses (Bech32 format), which are more efficient and cheaper.


- **BIP-173**: Base32 Address Format for SegWit Addresses (Bech32)  
  Specifies the Bech32 address format used for native SegWit addresses, improving efficiency and error detection.

## 2. Transaction and Script Standards
These BIPs focus on transaction structures, signing processes, and the Bitcoin scripting language:

- **BIP-16**: Pay to Script Hash (P2SH)  
  Introduces a new type of Bitcoin address format that allows for more complex scripts, enabling multi-signature wallets and other advanced features.

- **BIP-65**: CheckLockTimeVerify (CLTV)  
  Adds a new opcode to the Bitcoin scripting language that enables time-based conditional transactions (e.g., only allowing a transaction to be spent after a specific time).

- **BIP-68**: Relative Lock-Time using Consensus-Enforced Sequence Numbers  
  Introduces relative lock times, allowing for more complex transaction conditions, such as time-delayed transactions.

- **BIP-141**: Segregated Witness (SegWit)  
  SegWit separates the digital signature (witness) from the transaction data, solving the transaction malleability issue and allowing for larger block sizes.

- **BIP-143**: Transaction Signature Verification for Version 0 Witness Program  
  Defines a new method for signing SegWit transactions, improving efficiency and reducing malleability.

- **BIP-147**: Dealing with Dummy Stack Element Malleability  
  Specifies rules for SegWit to prevent certain types of malleability attacks related to dummy stack elements.

- **BIP-340, BIP-341, BIP-342**: Schnorr Signatures, Taproot, and Tapscript  
  These BIPs introduce Schnorr signatures, Taproot, and Tapscript, enhancing Bitcoin’s privacy, efficiency, and flexibility by optimizing multi-signature transactions and smart contracts.

## 3. Network and Communication Standards
These BIPs are focused on how Bitcoin nodes communicate and manage network activity:

- **BIP-37**: Connection Bloom Filtering  
  Implements bloom filters, which allow lightweight clients (SPV clients) to request relevant transactions from full nodes without revealing the addresses they control.

- **BIP-61**: Reject P2P Message  
  Adds a mechanism for nodes to reject invalid transactions or blocks and provide an error code, helping nodes understand why a transaction or block was rejected.

- **BIP-152**: Compact Block Relay  
  Introduces a more efficient method for nodes to relay blocks, reducing the bandwidth and latency needed to propagate blocks across the network.

- **BIP-157 and BIP-158**: Compact Block Filters for Lightweight Clients  
  Propose a method for lightweight clients to receive relevant information from the blockchain without downloading the entire block, improving efficiency.

## 4. Consensus Improvements
These BIPs are focused on improving Bitcoin’s consensus rules or introducing new features:

- **BIP-9**: Version Bits with Timeout and Delay  
  Specifies a mechanism for activating soft forks in a controlled manner using version bits, allowing for better coordination in the network.

- **BIP-34**: Block v2, Height in Coinbase  
  Changes the format of Bitcoin blocks to include the block height in the coinbase transaction, improving tracking and preventing certain forms of attack.

- **BIP-42**: A Max Number of Coins  
  Proposes a method to enforce the Bitcoin supply cap of 21 million coins, preventing inflation.

- **BIP-66**: Strict DER Encoding of Signatures  
  Ensures all signatures follow the DER encoding format, which is important for preventing transaction malleability.

- **BIP-141**: Segregated Witness (SegWit)  
  SegWit is not only a transaction and script standard but also a consensus rule change, improving scalability and enabling second-layer solutions like the Lightning Network.

## 5. User Experience and Interface Standards
These BIPs aim to improve user interactions with Bitcoin wallets and services:

- **BIP-21**: URI Scheme  
  Specifies a uniform resource identifier (URI) scheme for Bitcoin payment requests, allowing users to click links and open their wallets with pre-filled payment details.

- **BIP-70**: Payment Protocol  
  Describes a protocol for wallets and merchants to communicate securely during payment processes, providing more information and protection to both parties.

- **BIP-125**: Replace-By-Fee (RBF)  
  Allows users to increase the transaction fee for unconfirmed transactions, improving flexibility and ensuring that time-sensitive transactions can be confirmed more quickly.

## 6. Standardization BIPs
These BIPs provide the necessary technical and structural foundations for other BIPs:

- **BIP-1**: BIP Purpose and Guidelines  
  Describes the purpose of BIPs, the standards for submitting BIPs, and the guidelines for how they should be structured and formatted.

- **BIP-2**: BIP Process  
  Outlines the process for proposing, reviewing, and implementing BIPs, including stages like drafting, discussion, implementation, and activation.

- **BIP-123**: BIP Classification  
  Provides a classification system for BIPs (Standards Track, Informational, and Process), helping to organize and prioritize them based on their impact and purpose.

## Summary
Bitcoin Improvement Proposals cover a wide range of standards for wallets, transactions, consensus rules, network behavior, and user experience. These standards help evolve Bitcoin while maintaining consensus and compatibility across the network.
