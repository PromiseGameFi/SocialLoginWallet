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

impl Bip39Dictionary {
    /// Load the bip-39 dictionary from a file.
    pub fn load<P: AsRef<Path>>(dictionary_path: P) -> Result<Self> {
        let words = read_to_string(dictionary_path)?
            .lines()
            .map(Into::into)
            .collect::<Vec<_>>();
        let length = words.len();

        Ok(Self {
            words: words.try_into().map_err(|_| {
                eyre!("Invalid BIP-39 dictionary length {length} != {DICTIONARY_WORDS}")
            })?,
        })
    }

} 

fn reconstruct<S: AsRef<ShamirShare<Self>>>(shares: &[S]) -> Self {
    let mut y = gf256(0);
    for (i, share) in shares.iter().enumerate() {
        let mut li = gf256(1);
        let (x0, y0) = share.as_ref().as_coordinates();
        for (j, share) in shares.iter().enumerate() {
            let (x1, _y1) = share.as_ref().as_coordinates();
            if i != j {
                li *= gf256(*x1) / (gf256(*x0) + gf256(*x1));
            }
        }
        y += li * y0;
    }
    y
}