# Secret Sharing Algorithms Overview

## Popular Algorithms

1. **Shamir's Secret Sharing**
   - Based on polynomial interpolation
   - Widely used due to simplicity and flexibility

2. **Blakley's Scheme**
   - Uses hyperplane geometry
   - More storage, but can be computationally efficient

3. **Asmuth-Bloom**
   - Based on Chinese Remainder Theorem
   - Efficient for certain applications

4. **Additive Secret Sharing**
   - Shares sum up to the secret
   - Fast but requires all shares

5. **XOR Secret Sharing**
   - Uses XOR operation
   - All shares needed for reconstruction

6. **Krawczyk's Secret Sharing Made Short**
   - Combines encryption with secret sharing
   - Reduces share size for large secrets

7. **Feldman's Verifiable Secret Sharing (VSS)**
   - Allows share verification
   - Uses homomorphic encryption

8. **Pedersen's VSS**
   - Information-theoretic security
   - Uses two polynomials and discrete logarithms

9. **Proactive Secret Sharing**
   - Periodic share renewal
   - Protects against gradual corruption

10. **Quantum Secret Sharing**
    - Uses quantum states
    - Theoretical with growing practical implementations

11. **Visual Cryptography**
    - Shares are random-looking images
    - Physical overlay for reconstruction

## Selection Criteria

- Security level required
- Computational resources
- Storage constraints
- Verifiability needs
- Partial reconstruction tolerance
- Specific threat models

## Note
Shamir's remains popular, but alternatives offer specific benefits for certain use cases.
