pub fn process_encode(input: &str) -> anyhow::Result<()> {
    let encoded = base64::encode(input);
    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str) -> anyhow::Result<()> {
    let decoded = base64::decode(input)?;
    println!("{}", String::from_utf8(decoded));
    Ok(())
}
