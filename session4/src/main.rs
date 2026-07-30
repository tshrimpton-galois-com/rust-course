use session4::*;

fn main() -> Result<(), &'static str> {
    let key = SecretKey::<A>::new(vec![42]);
    let nonce1 = Nonce::<A>::new(7);

    let c = key.encrypt(nonce1, 100)?;
    println!("ciphertext: {}", c.value());

    let nonce2 = Nonce::<A>::new(7);
    let p = key.decrypt(nonce2, c);
    println!("plaintext: {}", p);

    Ok(())
}