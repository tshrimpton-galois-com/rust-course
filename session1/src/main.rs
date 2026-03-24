fn main() {
    let a = vec![1, 2, 3];
    let b = vec![1, 1, 2];

    match xor_bytes(&a, &b) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("Error: {}", e),
    }
}

fn xor_bytes(a: &[u8], b: &[u8]) -> Result<Vec<u8>, &'static str> {
    if a.len() != b.len() {
        return Err("length mismatch");
    }

    let result: Vec<u8> = a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect();

    Ok(result)
}

fn f() -> &'static str {
    // recall that string literals are stored in the binary, not on the program heap, so you can only reference them; a function cannot "own" a string literal because the ownership is at a higher level of abstraction (intuitively, it is "owned" by the program itself) and in static memory
    "hello"
}

#[test]
fn test_mismatch() {
    let a = vec![1, 2, 3];
    let b = vec![1, 2];

    assert!(xor_bytes(&a, &b).is_err());
}
