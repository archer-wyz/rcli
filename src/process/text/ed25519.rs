use crate::process::text::{KeyGenerate, TextSign, TextVerify};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519::Signature;
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::io::Read;

pub struct Ed25519Gen {}

impl Ed25519Gen {
    pub fn new() -> Self {
        Ed25519Gen {}
    }
}

impl KeyGenerate for Ed25519Gen {
    fn generate(&self) -> Result<HashMap<&'static str, Vec<u8>>, anyhow::Error> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        // let pk = (&sk).into();
        // 由于下方存在
        //      let pk = pk.to_bytes().to_vec();
        // pk 会被推导成 Vec<u8>，报错
        // |
        // |         let pk = (&sk).into();
        // |             ^^
        // |         let sk = sk.to_bytes().to_vec();
        // |         let pk = pk.to_bytes().to_vec();
        // |                  -- type must be known at this point
        let pk: VerifyingKey = (&sk).into();
        let sk = sk.to_bytes().to_vec();
        let pk = pk.to_bytes().to_vec();
        let mut keys = HashMap::new();
        keys.insert("ed25519.pk", pk);
        keys.insert("ed25519.sk", sk);
        Ok(keys)
    }
}

pub struct Ed25519Signer {
    key: SigningKey,
}

impl Ed25519Signer {
    pub fn new(key: [u8; 32]) -> Self {
        let key = SigningKey::from_bytes(&key);
        Ed25519Signer { key }
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<String> {
        let mut data = Vec::new();
        reader
            .read_to_end(&mut data)
            .map_err(|e| anyhow::anyhow!("Error reading data: {}", e))?;
        let signature = self.key.sign(&data);
        let signature = signature.to_bytes();
        Ok(URL_SAFE_NO_PAD.encode(signature))
    }
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl Ed25519Verifier {
    pub fn new(key: [u8; 32]) -> Self {
        let key = VerifyingKey::from_bytes(&key).unwrap();
        Ed25519Verifier { key }
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, signature: &str) -> anyhow::Result<bool> {
        let mut data = Vec::new();
        reader
            .read_to_end(&mut data)
            .map_err(|e| anyhow::anyhow!("Error reading data: {}", e))?;
        let signature = URL_SAFE_NO_PAD
            .decode(signature.as_bytes())
            .map_err(|e| anyhow::anyhow!("Error decoding signature: {}", e))?;
        let signature: [u8; 64] = signature
            .try_into()
            .map_err(|_| anyhow::anyhow!("Signature must be exactly 64 bytes long"))?;
        let signature = Signature::from_bytes(&signature);
        Ok(self.key.verify(&data, &signature).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_ed25519_sign_verify() {
        let gen = Ed25519Gen::new();
        let pks = gen.generate().unwrap();
        let pk = pks.get("ed25519.pk").unwrap().to_vec();
        let pk: [u8; 32] = pk.try_into().unwrap();
        let sk = pks.get("ed25519.sk").unwrap().to_vec();
        let sk: [u8; 32] = sk.try_into().unwrap();

        let signer = Ed25519Signer::new(sk);
        let verifier = Ed25519Verifier::new(pk);
        let data = b"hello world";

        let mut reader = Cursor::new(data);
        let signature = signer.sign(&mut reader).unwrap();
        let mut reader = Cursor::new(data);
        let result = verifier.verify(&mut reader, &signature).unwrap();
        assert!(result);
    }
}
