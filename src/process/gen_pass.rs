use ::rand;
use anyhow::Result;
use rand::seq::SliceRandom;

pub fn process_gen_pass(
    length: usize,
    count: usize,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
) -> Result<Vec<String>> {
    let mut rng = rand::thread_rng();
    let mut charset = Vec::new();
    if uppercase {
        charset.extend_from_slice(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if lowercase {
        charset.extend_from_slice(b"abcdefghijklmnopqrstuvwxyz");
    }
    if number {
        charset.extend_from_slice(b"0123456789");
    }
    if symbol {
        charset.extend_from_slice(b"!@#$%^&*()-_=+");
    }
    let mut passwords = Vec::with_capacity(count);
    for _ in 0..count {
        let mut password = String::with_capacity(length);
        for _ in 0..length {
            let c = charset.choose(&mut rng).expect("chars want be empty");
            password.push(*c as char);
        }
        passwords.push(password);
    }
    Ok(passwords)
}
