# Stateless Authentication with Encryption and Hashing

This README describes a method for implementing stateless authentication using encryption and hashing techniques. This approach allows for secure verification of information without storing it, which can be particularly useful for handling sensitive data like shares of a seed phrase split using Shamir's Secret Sharing.

## Overview

The process involves three main steps:

1. **Encryption**: Using a strong encryption algorithm with a secret key.
2. **Hashing**: Applying a cryptographic hash function to the encrypted data.
3. **Stateless Verification**: Creating and using tokens for verification instead of storing data.

## Process Flow

### Authentication
1. User provides credentials (e.g., username and password, or a Shamir share).
2. Encrypt the credentials using a secret key.
3. Hash the encrypted data.
4. Create a token including:
   - The hash
   - A timestamp
   - Any other necessary metadata
5. Sign the token using a secret key to ensure integrity.

### Verification
1. Receive the token.
2. Verify the token's signature to ensure it hasn't been tampered with.
3. Check if the token has expired based on the timestamp.
4. Recreate the hash using the provided credentials and compare it to the hash in the token.

## Implementation Example

Here's a basic Python implementation demonstrating the core concepts:

```python
import os
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.backends import default_backend

def encrypt_share(share, key):
    iv = os.urandom(16)
    cipher = Cipher(algorithms.AES(key), modes.GCM(iv), backend=default_backend())
    encryptor = cipher.encryptor()
    ciphertext = encryptor.update(share) + encryptor.finalize()
    return iv + encryptor.tag + ciphertext

def hash_encrypted_share(encrypted_share):
    digest = hashes.Hash(hashes.SHA256(), backend=default_backend())
    digest.update(encrypted_share)
    return digest.finalize()

def process_share(share, encryption_key):
    encrypted_share = encrypt_share(share, encryption_key)
    hashed_share = hash_encrypted_share(encrypted_share)
    return encrypted_share, hashed_share

# Usage example
encryption_key = os.urandom(32)
share = b"This is a Shamir share of the seed phrase"
encrypted_share, hashed_share = process_share(share, encryption_key)
```

## Security Considerations

While this approach provides a method for stateless authentication, it's important to consider several factors for a production-ready implementation:

- Secure key management
- Proper error handling
- Protection against timing attacks
- Secure communication channels
- Compliance with relevant security standards

## Note

This README provides a simplified explanation of the process. Implementing a secure authentication system requires careful consideration of many factors and should be reviewed by security experts before being used in a production environment.
