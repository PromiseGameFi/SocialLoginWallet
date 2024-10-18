### Seed Phrase Generation:

When you first create a wallet, a random seed phrase (also called a mnemonic phrase) is generated.
This seed phrase typically consists of 12 to 24 words from a predefined wordlist.


### Seed to Master Private Key:

The seed phrase is converted into a binary seed using a standardized algorithm (usually PBKDF2).
This binary seed is then used to generate the master private key.


### Hierarchical Deterministic (HD) Wallets:

Most modern cryptocurrency wallets use HD wallet structure (BIP32/BIP44 standards).
From the master private key, the wallet can derive multiple private keys for different cryptocurrencies or accounts.


### Private Key to Public Key:

For each private key, a corresponding public key is generated using elliptic curve cryptography.


### Public Key to Address:

The public key is then hashed to create the wallet address.