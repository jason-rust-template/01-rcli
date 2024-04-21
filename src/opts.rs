use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),
}

// default_value 是可以使用from trait 来进行解析
// default_value_t 是直接设置指定类型不用转换
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    //  form trait 里会干这件事情 "output.json".into()
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File {} does not exist")
    }
}
