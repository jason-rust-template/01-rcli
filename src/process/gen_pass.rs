use rand::seq::SliceRandom;
pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = String::new();
    let mut chars = Vec::new();
    if upper {
        chars.extend_from_slice(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ")
    }

    if lower {
        chars.extend_from_slice(b"abcdefghijklmnopqrstuvwxyz")
    }

    if number {
        chars.extend_from_slice(b"0123456789")
    }

    if symbol {
        chars.extend_from_slice(b"`~!@#$%^&*()_")
    }

    for _ in 0..length {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c as char);
    }

    println!("{}", password);

    Ok(())
}
