use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"~@#$%^&*()";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut pass = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        pass.push(
            *UPPER
                .choose(&mut rng)
                .expect("No characters left to choose from"),
        );
    }
    if lower {
        chars.extend_from_slice(LOWER);
        pass.push(
            *LOWER
                .choose(&mut rng)
                .expect("No characters left to choose from"),
        );
    }
    if number {
        chars.extend_from_slice(NUMBER);
        pass.push(
            *NUMBER
                .choose(&mut rng)
                .expect("No characters left to choose from"),
        );
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        pass.push(
            *SYMBOL
                .choose(&mut rng)
                .expect("No characters left to choose from"),
        );
    }

    for _ in 0..(length - pass.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("No characters left to choose from");
        pass.push(*c);
    }
    pass.shuffle(&mut rng);
    println!("{}", String::from_utf8(pass).unwrap());
    Ok(())
}
