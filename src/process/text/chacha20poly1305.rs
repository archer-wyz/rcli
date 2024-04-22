use super::{KeyGenerate, TextDecrypt, TextEncrypt};
use anyhow::{anyhow, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use std::collections::HashMap;
use std::io::Read;

pub struct ChaCha20Poly1305EnDe {
    cipher: ChaCha20Poly1305,
    nonce: Nonce,
}

impl ChaCha20Poly1305EnDe {
    pub fn try_new(nonce_key: &[u8]) -> Result<Self> {
        let (nonce, key) = ChaCha20Poly1305Generator::nonce_key(nonce_key)?;
        // 通过进入代码查看 impl_from! 的宏查看
        // 不支持传引用
        //  impl<T> From<[T; $n]> for GenericArray<T, $ty> {
        //                 #[inline(always)]
        //                 fn from(arr: [T; $n]) -> Self {
        //                     unsafe { $crate::transmute(arr) }
        //                 }
        //             }
        let key = Key::from(*key);
        Ok(ChaCha20Poly1305EnDe {
            cipher: ChaCha20Poly1305::new(&key),
            nonce: Nonce::from(*nonce),
        })
    }
}

impl TextEncrypt for ChaCha20Poly1305EnDe {
    fn encrypt(&self, reader: &mut dyn Read) -> Result<String> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let buffer = buffer.as_slice();
        let cipher_text = self
            .cipher
            .encrypt(&self.nonce, buffer)
            .map_err(|_| anyhow!("Encrypt failed"))?;
        Ok(URL_SAFE_NO_PAD.encode(cipher_text))
    }
}

impl TextDecrypt for ChaCha20Poly1305EnDe {
    fn decrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let buffer = URL_SAFE_NO_PAD.decode(&buffer)?;
        let plain_text = self
            .cipher
            .decrypt(&self.nonce, buffer.as_slice())
            .map_err(|_| anyhow!("Decrypt failed"))?;
        Ok(plain_text)
    }
}

pub struct ChaCha20Poly1305Generator {}

impl ChaCha20Poly1305Generator {
    pub fn new() -> Self {
        ChaCha20Poly1305Generator {}
    }
}

impl ChaCha20Poly1305Generator {
    fn nonce_key(nonce_key: &[u8]) -> Result<(&[u8; 12], &[u8; 32])> {
        if nonce_key.len() < 9 {
            return Err(anyhow!("Key length must greater than 12 bytes"));
        }
        Ok((
            <&[u8; 12]>::try_from(&nonce_key[..12])?,
            <&[u8; 32]>::try_from(&nonce_key[12..])?,
        ))
    }
}

impl KeyGenerate for ChaCha20Poly1305Generator {
    fn generate(&self) -> Result<HashMap<&'static str, Vec<u8>>> {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let mut nonce = nonce.to_vec();
        let key = key.to_vec();

        nonce.extend_from_slice(&key);

        let mut keys = HashMap::new();
        keys.insert("chacha20poly1305.key", nonce);

        Ok(keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_chacha20poly1305_generate() {
        let generator = ChaCha20Poly1305Generator::new();
        let keys = generator.generate().unwrap();
        assert_eq!(keys.len(), 1);
        assert!(keys.contains_key("chacha20poly1305.key"));
        let (nonce, key) =
            ChaCha20Poly1305Generator::nonce_key(keys.get("chacha20poly1305.key").unwrap())
                .unwrap();
        println!("{:?}", nonce);
        println!("{:?}", key);
    }

    #[test]
    fn test_chacha20poly1305_new() {
        let generator = ChaCha20Poly1305Generator::new();
        let keys = generator.generate().unwrap();
        ChaCha20Poly1305Generator::nonce_key(keys.get("chacha20poly1305.key").unwrap()).unwrap();
        ChaCha20Poly1305EnDe::try_new(keys.get("chacha20poly1305.key").unwrap()).unwrap();
    }

    #[test]
    fn test_chacha20poly1305_encrypt_decrypt() {
        let generator = ChaCha20Poly1305Generator::new();
        let keys = generator.generate().unwrap();
        let en_de =
            ChaCha20Poly1305EnDe::try_new(keys.get("chacha20poly1305.key").unwrap()).unwrap();

        let data = b"hello world!";
        let mut reader = Cursor::new(data);
        let cipher_text = en_de.encrypt(&mut reader).unwrap();
        let mut reader = Cursor::new(cipher_text);
        let plain_text = en_de.decrypt(&mut reader).unwrap();

        println!("{:?}", String::from_utf8(plain_text.clone()).unwrap());
        assert_eq!(plain_text, "hello world!".as_bytes());
    }
}
