// rcli csv -i input.csv -o output.csv --header -d ','
use clap::Parser;
use rcli::{process_csv, process_genpass, Base64SubCommand, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = &opts.output {
                output.clone()
            } else {
                format!("{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                println!("{:?}", opts);
            }
            Base64SubCommand::Decode(opts) => {
                println!("{:?}", opts);
            }
        },
    }
    Ok(())
}
