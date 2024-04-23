use crate::cli::JwtAlg;
use anyhow::Result;
use chrono::{Duration, Utc};
use hmac::Mac;
use jwt::{AlgorithmType, Header, SignWithKey, SigningAlgorithm, Token};
use sha2::{Sha256, Sha384};
use std::collections::BTreeMap;

pub fn process_jwt_sign(
    sub: &str,
    aud: &str,
    exp: Duration,
    alg: JwtAlg,
    key: &str,
) -> Result<String> {
    let mut claims = BTreeMap::new();
    claims.insert("sub", sub);
    claims.insert("aud", aud);
    let now = Utc::now();
    let exp_at = now + exp;
    let exp_at = exp_at.to_string();
    claims.insert("exp_at", exp_at.as_str());

    let header = Header::from(alg);
    let sign_key = alg.new_from_slice(key.as_bytes())?;
    let token = Token::new(header, claims).sign_with_key(&sign_key)?;
    Ok(token.as_str().to_string())
}

impl From<JwtAlg> for Header {
    fn from(alg: JwtAlg) -> Self {
        match alg {
            JwtAlg::HS256 => Header {
                algorithm: AlgorithmType::Hs256,
                ..Default::default()
            },
            JwtAlg::HS384 => Header {
                algorithm: AlgorithmType::Hs384,
                ..Default::default()
            },
        }
    }
}

impl JwtAlg {
    fn new_from_slice(&self, key: &[u8]) -> Result<Box<dyn SigningAlgorithm>> {
        match self {
            JwtAlg::HS256 => Ok(Box::new(hmac::Hmac::<Sha256>::new_from_slice(key)?)),
            JwtAlg::HS384 => Ok(Box::new(hmac::Hmac::<Sha384>::new_from_slice(key)?)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process_jwt_sign() {
        let sub = "test";
        let aud = "test";
        let exp = Duration::days(1);
        let alg = JwtAlg::HS256;
        let key = "test";
        let token = process_jwt_sign(sub, aud, exp, alg, key);
        assert!(token.is_ok());
        println!("Token: {}", token.unwrap())
    }
}
