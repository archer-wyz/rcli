use super::data_from_input;
use crate::cli::CryptFormat;
use crate::process_gen_pass;
use anyhow::Result;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use blake3;
use std::collections::HashMap;
use std::fs;
use std::io::Read;

pub fn process_text_sign(input: &str, key: &str, format: CryptFormat) -> Result<String> {
    println!("input: {}, key: {}, format: {:?}", input, key, format);
    let mut reader = data_from_input(input)?;
    let key = fs::read(key)?;
    let signer: Box<dyn TextSign> = match format {
        CryptFormat::BlakeCrypt => {
            let key = key.try_into().expect("Key must be exactly 32 bytes long");
            Box::new(BlakeSign::new(key))
        }
    };
    signer.sign(&mut reader)
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    signature: &str,
    format: CryptFormat,
) -> Result<bool> {
    let mut reader = data_from_input(input)?;
    let key = fs::read(key)?;
    let verifier: Box<dyn TextVerify> = match format {
        CryptFormat::BlakeCrypt => {
            let key = key.try_into().expect("Key must be exactly 32 bytes long");
            Box::new(BlakeVerify::new(key))
        }
    };
    verifier.verify(&mut reader, signature)
}

pub fn process_text_generate(format: CryptFormat) -> Result<HashMap<&'static str, Vec<u8>>> {
    let generator: Box<dyn KeyGenerate> = match format {
        CryptFormat::BlakeCrypt => Box::new(BlakeGenerate {}),
    };
    generator.generate()
}

trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<String>;
}

trait TextVerify {
    fn verify(&self, data: &mut dyn Read, signature: &str) -> Result<bool>;
}

trait KeyGenerate {
    fn generate(&self) -> Result<HashMap<&'static str, Vec<u8>>>;
}

struct BlakeSign {
    key: [u8; 32],
}

struct BlakeVerify {
    key: [u8; 32],
}

struct BlakeGenerate {}

impl BlakeVerify {
    fn new(key: [u8; 32]) -> Self {
        BlakeVerify { key }
    }

    #[cfg(test)]
    fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        // convert &[u8] to &[u8; 32]
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }
    #[cfg(test)]
    fn load(key: &str) -> Result<Self> {
        let key = fs::read(key)?;
        BlakeVerify::try_new(key)
    }
}

impl BlakeSign {
    fn new(key: [u8; 32]) -> Self {
        BlakeSign { key }
    }

    #[cfg(test)]
    fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        // convert &[u8] to &[u8; 32]
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    #[cfg(test)]
    fn load(key: &str) -> Result<Self> {
        let key = fs::read(key)?;
        BlakeSign::try_new(key)
    }
}

impl TextSign for BlakeSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<String> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let hasher = blake3::keyed_hash(&self.key, &data);
        let hasher = hasher.as_bytes();
        let hasher = hasher.to_vec();
        Ok(URL_SAFE_NO_PAD.encode(hasher))
    }
}

impl TextVerify for BlakeVerify {
    fn verify(&self, reader: &mut dyn Read, signature: &str) -> Result<bool> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let hasher = blake3::keyed_hash(&self.key, &data);
        let hasher = hasher.as_bytes();
        let hasher = URL_SAFE_NO_PAD.encode(hasher);
        Ok(hasher == signature)
    }
}

impl KeyGenerate for BlakeGenerate {
    fn generate(&self) -> Result<HashMap<&'static str, Vec<u8>>> {
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
