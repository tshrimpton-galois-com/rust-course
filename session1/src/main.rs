fn main() {
    let a = vec![1, 2, 3];
    let b = vec![1, 1, 1];

    match xor_bytes(&a, &b) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("Error: {}", e),
    }
}

fn xor_bytes(a: &[u8], b: &[u8]) -> Result<Vec<u8>, String> {
    if a.len() != b.len() {
        return Err("length mismatch".to_string());
    }
    let result: Vec<u8> = a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect();

    Ok(result)
}
