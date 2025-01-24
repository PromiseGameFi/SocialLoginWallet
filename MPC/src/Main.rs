// SPDX-License-Identifier: Apache-2.0

mod bip39;
mod gf256;
mod shamir;
mod utils;



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
   
}

impl<T> From<&Entropy> for FieldArray<T, ENTROPY_BYTES>
where
    T: From<u8> + Debug,
{
    
}

/// The checksum of a bip-39 secret.
#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Clone, Debug))]
struct Checksum([bool; CHECKSUM_BITS]);

impl TryFrom<&[bool]> for Checksum {
    type Error = TryFromSliceError;

    fn try_from(value: &[bool]) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
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



