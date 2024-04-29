use clap::{arg, Parser};

use super::verify_input_file;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value="-" )]
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long,  value_parser = verify_input_file, default_value="-")]
    pub input: String,
}
