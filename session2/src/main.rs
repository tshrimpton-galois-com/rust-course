fn first_byte(data: &[u8]) -> Option<u8> {
    if data.len() == 0 { None } else { Some(data[0]) }
}

fn parse_number(s: &str) -> Result<u32, &'static str> {
    if s == "42" { Ok(42) } else { Err("not 42") }
}

fn double_number(s: &str) -> Result<u32, &'static str> {
    let x = parse_number(s)?;
    Ok(2 * x)
}

fn main() -> Result<(), &'static str> {
    println!("{:?}", double_number("42"));
    println!("{:?}", double_number("7"));

    let a = vec![1, 2, 3];
    let b = vec![1, 2];
    let v = xor_bytes(&a, &b)?;
    println!("{:?}", v);
    Ok(())
}

fn check_len(a: &[u8], b: &[u8]) -> Result<(), &'static str> {
    if a.len() != b.len() {
        Err("length mismatch")
    } else {
        Ok(())
    }
}

fn xor_bytes(a: &[u8], b: &[u8]) -> Result<Vec<u8>, &'static str> {
    check_len(a, b)?;
    let result: Vec<u8> = a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect();
    Ok(result)
}
