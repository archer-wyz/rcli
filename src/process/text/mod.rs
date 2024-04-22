use crate::cli::CryptFormat;
use anyhow::Result;
use std::collections::HashMap;
use std::io::Read;

mod blake;
mod ed25519;

pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<String>;
}

pub trait TextVerify {
    fn verify(&self, data: &mut dyn Read, signature: &str) -> anyhow::Result<bool>;
}

pub trait KeyGenerate {
    fn generate(&self) -> anyhow::Result<HashMap<&'static str, Vec<u8>>>;
}

pub fn create_signer(
    format: CryptFormat,
    key: Vec<u8>,
) -> Result<Box<dyn TextSign>, anyhow::Error> {
    match format {
        CryptFormat::BlakeCrypt => {
            let key = key
                .try_into()
                .map_err(|_| anyhow::anyhow!("Blake's key must be exactly 32 bytes long"))?;
            Ok(Box::new(blake::BlakeSign::new(key)))
        }
        CryptFormat::Ed25519Crypt => {
            let key = key
                .try_into()
                .map_err(|_| anyhow::anyhow!("Ed25519's key must be exactly 32 bytes long"))?;
            Ok(Box::new(ed25519::Ed25519Signer::new(key)))
        }
    }
}

pub fn create_verifier(
    format: CryptFormat,
    key: Vec<u8>,
) -> Result<Box<dyn TextVerify>, anyhow::Error> {
    match format {
        CryptFormat::BlakeCrypt => {
            let key = key
                .try_into()
                .map_err(|_| anyhow::anyhow!("Blake's key must be exactly 32 bytes long"))?;
            Ok(Box::new(blake::BlakeVerify::new(key)))
        }
        CryptFormat::Ed25519Crypt => {
            let key = key
                .try_into()
                .map_err(|_| anyhow::anyhow!("Ed25519's key must be exactly 32 bytes long"))?;
            Ok(Box::new(ed25519::Ed25519Verifier::new(key)))
        }
    }
}

pub fn create_generator(format: CryptFormat) -> Result<Box<dyn KeyGenerate>, anyhow::Error> {
    match format {
        CryptFormat::BlakeCrypt => Ok(Box::new(blake::BlakeGenerate {})),
        CryptFormat::Ed25519Crypt => Ok(Box::new(ed25519::Ed25519Gen::new())),
    }
}
