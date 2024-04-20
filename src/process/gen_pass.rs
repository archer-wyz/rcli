use ::rand;
use anyhow::Result;
use rand::prelude::IndexedRandom;

pub fn process_gen_pass(
    length: usize,
    count: usize,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
) -> Result<()> {
    let mut rng = rand::thread_rng();
    let mut charset = Vec::new();
    if uppercase {
        charset.extend_from_slice(b"ABCDEFGHIJKLMNPQRSTUVWXYZ");
    }
    if lowercase {
        charset.extend_from_slice(b"abcdefghijkmnopqrstuvwxyz");
    }
    if number {
        charset.extend_from_slice(b"0123456789");
    }
    if symbol {
        charset.extend_from_slice(b"!@#$%^&*()-_=+");
    }
    let mut password = String::with_capacity(length);
    for _ in 0..count {
        for _ in 0..length {
            let c = charset.choose(&mut rng).expect("chars want be empty");
            password.push(*c as char);
        }
        println!("{}", password);
    }

    let estimate = zxcvbn::zxcvbn(&password, &[]).unwrap();
    eprintln!("The password score: {}", estimate.score());
    Ok(())
}
