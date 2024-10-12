# ECC and ECDH Overview

ECC (Elliptic Curve Cryptography) and ECDH (Elliptic Curve Diffie-Hellman) are cryptographic protocols that rely on the mathematics of elliptic curves to provide secure communication. This document explains each protocol in detail.

## 1. ECC (Elliptic Curve Cryptography)

ECC is a type of public-key cryptography based on the algebraic structure of elliptic curves over finite fields. It offers a higher level of security with smaller key sizes compared to other algorithms like RSA, making it efficient and lightweight. 

### Benefits of ECC:
- **Smaller Key Size**: ECC can provide the same level of security as RSA with a much smaller key size (e.g., a 256-bit ECC key is considered as secure as a 3072-bit RSA key).
- **Performance Efficiency**: Due to the smaller key size, ECC requires less computational power and memory, making it ideal for devices with limited resources.

## 2. ECDH (Elliptic Curve Diffie-Hellman)

ECDH is a key exchange protocol based on ECC that allows two parties to securely exchange cryptographic keys over an insecure channel. It is the elliptic curve variant of the traditional Diffie-Hellman key exchange but offers the same level of security with much smaller keys, thanks to ECC.

### How ECDH Works:
1. Alice and Bob agree on a public elliptic curve and a point on the curve (known parameters).
2. Alice generates a private key and computes a public key by multiplying the private key with the agreed point on the curve.
3. Bob does the same, creating his own private and public keys.
4. They exchange public keys.
5. Alice computes the shared secret using her private key and Bob's public key, and Bob does the same using his private key and Alice’s public key.
6. Both arrive at the same shared secret, which can now be used for secure encryption.

## Key Takeaways:
- ECC provides a secure and efficient foundation for public-key cryptography.
- ECDH is a specific application of ECC used for secure key exchange.
