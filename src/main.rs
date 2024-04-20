use std::path::Path;
// rcli csv -i input.csv -o output.csv --header -d ','
use clap::Parser;
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: Subcommand,
}

#[derive(Debug, Parser)]
enum Subcommand {
    #[command(name = "csv", about = "show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),
}

// default_value 是可以使用from trait 来进行解析
// default_value_t 是直接设置指定类型不用转换
#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,
    //  form trait 里会干这件事情 "output.json".into()
    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
    #[arg(long, default_value_t = true)]
    header: bool,
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File {} does not exist".into())
    }
}
