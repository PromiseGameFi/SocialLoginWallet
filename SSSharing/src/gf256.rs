
//  SPDX-License-Identifier: Apache-2.0

use gf256::gf256;
use rand::{CryptoRng, Rng, RngCore};

use crate::shamir::{Random, ShamirPolynomial, ShamirSecretSharing, ShamirShare, Zero};

impl Zero for gf256 {
    fn zero() -> Self {
        gf256(0)
    }
}


        }
        y
    }
}

/// NOTE: No chaos test is implemented for the group GF(256) because the field is too small
/// to prevent collisions. 
#[cfg(test)]
mod test {
    use gf256::gf256;

    use crate::shamir;

    #[test]
    fn reconstruct() {
        shamir::test::test_reconstruct::<gf256>();
    }

    #[test]
    fn reconstruct_sparse() {
        shamir::test::test_reconstruct_sparse::<gf256>();
    }

    #[test]
    fn reconstruct_missing_shares() {
        shamir::test::test_reconstruct_missing_shares::<gf256>();
    }
}
