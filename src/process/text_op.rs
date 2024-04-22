use super::data_from_input;
use crate::cli::CryptFormat;
use crate::process::text::{
    create_decryptor, create_encryptor, create_generator, create_signer, create_verifier,
};
use anyhow::Result;
use std::collections::HashMap;
use std::fs;

pub fn process_text_sign(input: &str, key: &str, format: CryptFormat) -> Result<String> {
    println!("input: {}, key: {}, format: {:?}", input, key, format);
    let mut reader = data_from_input(input)?;
    let key = fs::read(key)?;
    let signer = create_signer(format, key)?;
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
    let verifier = create_verifier(format, key)?;
    verifier.verify(&mut reader, signature)
}

pub fn process_text_generate(format: CryptFormat) -> Result<HashMap<&'static str, Vec<u8>>> {
    let generator = create_generator(format)?;
    generator.generate()
}

pub fn process_text_encrypt(input: &str, key: &str, format: CryptFormat) -> Result<String> {
    let mut reader = data_from_input(input)?;
    let key = fs::read(key)?;
    let encryptor = create_encryptor(format, key)?;
    encryptor.encrypt(&mut reader)
}
pub fn process_text_decrypt(input: &str, key: &str, format: CryptFormat) -> Result<Vec<u8>> {
    let mut reader = data_from_input(input)?;
    let key = fs::read(key)?;
    let decryptor = create_decryptor(format, key)?;
    decryptor.decrypt(&mut reader)
}
