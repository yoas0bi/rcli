use anyhow::Ok;
use clap::Parser;
use rcli::{process_csv, process_decode, process_encode, process_genpass, Opts, SubCommand};
// rcli csv -i input.csv -o output.json --header -d ','

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
            Ok(())
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            Ok(())
        }
        SubCommand::Base64(base64_sub_command) => match base64_sub_command {
            rcli::Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
                Ok(())
            }
            rcli::Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
                Ok(())
            }
        },
    }
}
