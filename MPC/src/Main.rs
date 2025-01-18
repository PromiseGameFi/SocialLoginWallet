// SPDX-License-Identifier: Apache-2.0

mod bip39;
mod gf256;
mod shamir;
mod utils;

// SPDX-License-Identifier: Apache-2.0

use std::{array::TryFromSliceError, fmt::Debug, fs::read_to_string, path::Path};

use eyre::{ensure, eyre, Result};
use fastcrypto::hash::{HashFunction, Sha256};
use gf256::gf256;
use rand::{CryptoRng, RngCore};

use crate::{
    shamir::{FieldArray, ShamirSecretSharing, ShamirShare},
    utils::{bits_to_bytes, bytes_to_bits},
};

/// Parameters of the bip-39 specification (24 words variant).
const DICTIONARY_INDICES_BITS: usize = 11;
const MNEMONIC_WORDS: usize = 24;
const DICTIONARY_WORDS: usize = 2 << (DICTIONARY_INDICES_BITS - 1);
const CHECKSUM_BITS: usize = (MNEMONIC_WORDS * DICTIONARY_INDICES_BITS) / 33;
const ENTROPY_BITS: usize = CHECKSUM_BITS * 32;
const ENTROPY_BYTES: usize = ENTROPY_BITS / 8;

pub struct Bip39Dictionary {
    words: [String; DICTIONARY_WORDS],
}

#[test]
    fn test_bytes_to_bits() {
        assert_eq!(
            bytes_to_bits(&[0b0000_0000, 0b1111_1111]),
            vec![
                false, false, false, false, false, false, false, false, true, true, true, true,
                true, true, true, true
            ]
        );
        assert_eq!(
            bytes_to_bits(&[0b1010_1010]),
            vec![true, false, true, false, true, false, true, false]
        );
    }

    #[test]
    fn test_bits_to_bytes() {
        assert_eq!(
            bits_to_bytes(&[
                false, false, false, false, false, false, false, false, true, true, true, true,
                true, true, true, true
            ]),
            vec![0b0000_0000, 0b1111_1111]
        );
        assert_eq!(
            bits_to_bytes(&[true, false, true, false, true, false, true, false]),
            vec![0b1010_1010]
        );
    }

} 

