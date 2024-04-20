use ::anyhow;
use ::clap::Parser;
use rcli::{
    process_base64_decode, process_base64_encode, process_csv, process_gen_pass, Base64SubCommand,
    Opts, SubCommand,
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
        SubCommand::GenPass(gen_pass_opts) => process_gen_pass(
            gen_pass_opts.length,
            gen_pass_opts.count,
            gen_pass_opts.uppercase,
            gen_pass_opts.lowercase,
            gen_pass_opts.number,
            gen_pass_opts.symbol,
        )?,
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
    }
    Ok(())
}
