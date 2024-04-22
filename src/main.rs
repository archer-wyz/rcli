use ::anyhow;
use ::clap::Parser;
use rcli::{
    process_base64_decode, process_base64_encode, process_csv, process_gen_pass,
    process_text_decrypt, process_text_encrypt, process_text_generate, process_text_sign,
    process_text_verify, Base64SubCommand, Opts, SubCommand, TextSubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            let output = if let Some(output) = csv_opts.output {
                output.clone()
            } else {
                format!("output.{}", csv_opts.format)
            };
            process_csv(&csv_opts.input, output, csv_opts.format)?
        }
        SubCommand::GenPass(gen_pass_opts) => {
            let passwords = process_gen_pass(
                gen_pass_opts.length,
                gen_pass_opts.count,
                gen_pass_opts.uppercase,
                gen_pass_opts.lowercase,
                gen_pass_opts.number,
                gen_pass_opts.symbol,
            )?;
            for password in passwords {
                println!("{}", password);
            }
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(base64_opts) => {
                let result = process_base64_encode(&base64_opts.input, base64_opts.format)?;
                println!("{}", result);
            }
            Base64SubCommand::Decode(base64_opts) => {
                let result = process_base64_decode(&base64_opts.input, base64_opts.format)?;
                println!("{}", result);
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(sign_opts) => {
                let result = process_text_sign(&sign_opts.input, &sign_opts.key, sign_opts.format)?;
                println!("{}", result);
            }
            TextSubCommand::Verify(verify_opts) => {
                let result = process_text_verify(
                    &verify_opts.input,
                    &verify_opts.key,
                    &verify_opts.signature,
                    verify_opts.format,
                )?;
                println!("{}", result);
            }
            TextSubCommand::Generate(gen_opts) => {
                let result = process_text_generate(gen_opts.format)?;
                for (key, value) in result {
                    println!("{}: {:?}", key, value);
                    let path = gen_opts.output.join(key);
                    ::std::fs::write(path, value)?;
                }
            }
            TextSubCommand::Encrypt(encrypt_opts) => {
                let result = process_text_encrypt(
                    &encrypt_opts.input,
                    &encrypt_opts.key,
                    encrypt_opts.format,
                )?;
                println!("{}", result);
            }
            TextSubCommand::Decrypt(decrypt_opts) => {
                let result = process_text_decrypt(
                    &decrypt_opts.input,
                    &decrypt_opts.key,
                    decrypt_opts.format,
                )?;
                let result = String::from_utf8(result).map_err(|_| {
                    anyhow::anyhow!("Decrypt successfully, but the payloads exist invalid UTF-8")
                })?;
                println!("{}", result);
            }
        },
    }
    Ok(())
}
