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

/// Get the index of a word in the dictionary (as bits).
pub fn bits_from_word(&self, word: &str) -> Result<[bool; DICTIONARY_INDICES_BITS]> {
    let index = self
        .words
        .iter()
        .position(|w| w == word)
        .ok_or(eyre!("Invalid BIP-39 word '{word}' in mnemonic"))?;
    let bits = bytes_to_bits(&index.to_be_bytes());
    Ok(bits[bits.len() - DICTIONARY_INDICES_BITS..]
        .try_into()
        .expect("Slice size should match the dictionary index bit length"))
}

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



