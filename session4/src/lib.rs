use std::marker::PhantomData;

pub struct SecretKey<K>(Vec<u8>, PhantomData<K>);
pub struct Nonce<K>(u8, PhantomData<K>);
pub struct Ciphertext<K>(u8, PhantomData<K>);

pub struct A;
pub struct B;

impl<K> Ciphertext<K> {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl<K> SecretKey<K> {
    pub fn new(bytes: Vec<u8>) -> Self {
        SecretKey(bytes, PhantomData)
    }

    pub fn encrypt(&self, nonce: Nonce<K>, plaintext: u8) -> Result<Ciphertext<K>, &'static str> {
        if plaintext == 0 {
            return Err("zero plaintext not allowed");
        }

        let key_byte = self.0[0];
        Ok(Ciphertext(plaintext ^ key_byte ^ nonce.0, PhantomData))
    }

    pub fn decrypt(&self, nonce: Nonce<K>, ciphertext: Ciphertext<K>) -> u8 {
        let key_byte = self.0[0];
        ciphertext.0 ^ key_byte ^ nonce.0
    }
}

impl<K> Nonce<K> {
    pub fn new(x: u8) -> Self {
        Nonce(x, PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let key = SecretKey::<A>::new(vec![42]);
        let nonce = Nonce::<A>::new(7);

        let c = key.encrypt(nonce, 100).unwrap();
        let nonce2 = Nonce::<A>::new(7);
        let p = key.decrypt(nonce2, c);

        assert_eq!(p, 100);
    }

    #[test]
    fn zero_plaintext_rejected() {
        let key = SecretKey::<A>::new(vec![42]);
        let nonce = Nonce::<A>::new(7);

        let result = key.encrypt(nonce, 0);

        assert!(result.is_err());
    }
}