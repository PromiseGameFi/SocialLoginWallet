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

impl Entropy {
    pub fn as_bits(&self) -> &[bool] {
        &self.0
    }

    pub fn to_bytes(&self) -> [u8; ENTROPY_BYTES] {
        bits_to_bytes(&self.0).try_into().unwrap()
    }

    #[cfg(test)]
    pub fn random<R: CryptoRng + RngCore>(rng: &mut R) -> Self {
        use rand::Rng;

        Self(std::array::from_fn(|_| rng.gen()))
    }
}

impl TryFrom<&[bool]> for Entropy {
    type Error = TryFromSliceError;

    fn try_from(value: &[bool]) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl<T> From<FieldArray<T, ENTROPY_BYTES>> for Entropy
where
    u8: From<T>,
{
    fn from(value: FieldArray<T, ENTROPY_BYTES>) -> Self {
        let bytes = value.into_iter().map(u8::from).collect::<Vec<_>>();
        bytes_to_bits(&bytes).as_slice().try_into().unwrap()
    }
}

impl<T> From<&Entropy> for FieldArray<T, ENTROPY_BYTES>
where
    T: From<u8> + Debug,
{
    fn from(value: &Entropy) -> Self {
        value.to_bytes().map(T::from).into()
        value.to_be_bytes().map
    }
}



impl From<&Entropy> for Checksum {
    fn from(entropy: &Entropy) -> Self {
        let digest = Sha256::digest(entropy.to_bytes());
        let bits = bytes_to_bits(digest.as_ref());
        let checksum = bits[..CHECKSUM_BITS]
            .try_into()
            .expect("Slice size should match the checksum bit length");
        Self(checksum)
    }
}



