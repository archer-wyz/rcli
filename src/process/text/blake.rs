use super::{KeyGenerate, TextSign, TextVerify};
use crate::process_gen_pass;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use std::collections::HashMap;
use std::io::Read;

pub struct BlakeSign {
    key: [u8; 32],
}

pub struct BlakeVerify {
    key: [u8; 32],
}

pub struct BlakeGenerate {}

impl BlakeVerify {
    pub fn new(key: [u8; 32]) -> Self {
        BlakeVerify { key }
    }

    #[cfg(test)]
    fn try_new(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref();
        // convert &[u8] to &[u8; 32]
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }
    #[cfg(test)]
    fn load(key: &str) -> anyhow::Result<Self> {
        let key = std::fs::read(key)?;
        BlakeVerify::try_new(key)
    }
}

impl BlakeSign {
    pub fn new(key: [u8; 32]) -> Self {
        BlakeSign { key }
    }

    #[cfg(test)]
    fn try_new(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref();
        // convert &[u8] to &[u8; 32]
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    #[cfg(test)]
    fn load(key: &str) -> anyhow::Result<Self> {
        let key = std::fs::read(key)?;
        BlakeSign::try_new(key)
    }
}

impl TextSign for BlakeSign {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<String> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let hasher = blake3::keyed_hash(&self.key, &data);
        let hasher = hasher.as_bytes();
        let hasher = hasher.to_vec();
        Ok(URL_SAFE_NO_PAD.encode(hasher))
    }
}

impl TextVerify for BlakeVerify {
    fn verify(&self, reader: &mut dyn Read, signature: &str) -> anyhow::Result<bool> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let hasher = blake3::keyed_hash(&self.key, &data);
        let hasher = hasher.as_bytes();
        let hasher = URL_SAFE_NO_PAD.encode(hasher);
        Ok(hasher == signature)
    }
}

impl KeyGenerate for BlakeGenerate {
    fn generate(&self) -> anyhow::Result<HashMap<&'static str, Vec<u8>>> {
        let keys = process_gen_pass(32, 1, true, true, true, true)?;
        let mut key_map = HashMap::new();
        if keys.len() == 1 {
            key_map.insert("blake3.key", keys[0].as_bytes().to_vec());
        } else {
            unreachable!("Failed to generate key")
        };
        Ok(key_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_blake_sign_and_verify() {
        let signer = BlakeSign::load("fixtures/blake3_test/key.txt").expect("Failed to load key");
        let verifier =
            BlakeVerify::load("fixtures/blake3_test/key.txt").expect("Failed to load key");
        let data = b"hello world";
        let mut reader = Cursor::new(data);
        let signature = signer.sign(&mut reader).unwrap();
        let mut reader = Cursor::new(data);
        let verified = verifier.verify(&mut reader, &signature).unwrap();
        assert!(verified);
    }

    #[test]
    fn test_blake_generate() {
        let generator = BlakeGenerate {};
        let keys = generator.generate().unwrap();
        assert_eq!(keys.len(), 1);
        let key = keys.get("blake3.key").unwrap();
        println!("{:?}", String::from_utf8(key.to_vec()).unwrap());
        assert_eq!(key.len(), 32);
    }
}
