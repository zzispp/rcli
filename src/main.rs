use clap::Parser;
use rcli::{Base64SubCommand, Opts, SubCommand, process_csv, process_genpass,process_decode,process_encode};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            println!("{}", output.clone());
            process_csv(&opts.input, output, opts.format)?
        }
        SubCommand::GenPass(opts) => process_genpass(
            opts.length,
            opts.uppercase,
            opts.lowercase,
            opts.number,
            opts.symbol,
        )?,
        SubCommand::Base64(subcommand) => match subcommand {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input,opts.format)?
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input,opts.format)?
            }
        },
    }

    Ok(())
}
