use ::anyhow;
use ::clap::Parser;
use rcli::{process_csv, process_gen_pass, Opts, SubCommand};

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
        SubCommand::Base64(base64_opts) => {
            println!("base64: {:?}", base64_opts)
        }
    }
    Ok(())
}
