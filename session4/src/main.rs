use std::marker::PhantomData;

// Define (zero-sized) marker types for State
struct Init;
struct SentKey;
struct Established;

// Handshake structure
struct Handshake<State> {
    key: u32,
    _state: PhantomData<State>,
}

impl Handshake<Init> {
    fn new() -> Self {
        Handshake {
            key: 0,
            _state: PhantomData,
        }
    }

    fn send_key(self) -> Handshake<SentKey> {
        Handshake {
            key: 42,
            _state: PhantomData,
        }
    }
}

impl Handshake<SentKey> {
    fn establish(self) -> Handshake<Established> {
        Handshake {
            key: self.key,
            _state: PhantomData,
        }
    }
}

impl Handshake<Established> {
    fn encrypt(&self, x: u32) -> u32 {
        x ^ self.key
    }

    fn decrypt(&self, x: u32) -> u32 {
        x ^ self.key
    }

    fn maybe_encrypt(&self, x: u32) -> Option<u32> {
        if x == 0 { None } else { Some(self.encrypt(x)) }
    }

    fn maybe_decrypt(&self, x: u32) -> Option<u32> {
        if x == 0 { None } else { Some(self.decrypt(x)) }
    }
}

fn double_if_valid(x: u32) -> Option<u32> {
    if x == 0 { None } else { Some(x * 2) }
}

fn safe_double(x: u32) -> Result<u32, &'static str> {
    if x == 0 {
        Err("safe_double error: zero not allowed")
    } else {
        Ok(2 * x)
    }
}

fn process(x: u32) -> Result<u32, &'static str> {
    let y = safe_double(x)?;
    Ok(y + 1)
}

fn process2(x: u32) -> Result<u32, &'static str> {
    let y = safe_double(x)?;
    let z = safe_double(y)?;
    Ok(z)
}

/* --------- nonce-misuse example -------- */
struct Key(u8);
struct Nonce(u8);
struct Ciphertext(u8);

// methods for type Key
impl Key {
    // using nonce: Nonce consumes the input, so cannot be reused elsewhere; nonce: &Nonce would allow reuse
    // fn encrypt(self: &Key, nonce: Nonce, plaintext: u8) -> Ciphertext {
    fn encrypt(&self, nonce: Nonce, plaintext: u8) -> Ciphertext {
        //self: type Key(u8), self.0: first field in type Key(u8)
        Ciphertext(plaintext ^ self.0 ^ nonce.0)
    }

    fn decrypt(&self, nonce: Nonce, ciphertext: Ciphertext) -> u8 {
        ciphertext.0 ^ self.0 ^ nonce.0
    }
}

// methods for type Nonce
impl Nonce {
    //Nonce self-constructor
    fn new(x: u8) -> Self {
        Nonce(x)
    }

    fn fresh(x: u8) -> Self {
        Nonce(x)
    }
}

fn main() {
    let key = Key(42);
    let nonce = Nonce::fresh(7);
    println!("nonce is: {}", nonce.0);

    let nonce1 = Nonce::new(7);

    let c = key.encrypt(nonce1, 100);
    println!("ciphertext: {}", c.0);

    let nonce2 = Nonce(7);
    let p = key.decrypt(nonce2, c);
    println!("plaintext: {}", p);
}

/* --------- MAIN ---------- */
/*
static ptxt: u32 = 7;
fn main() {
    let h = Handshake::<Init>::new().send_key().establish();

    // "If Some(c), transform it; if None, keep None", i.e., map(f) transforms iff Option<T> succeeds
    // |c| a "variable closure", like an anonymous function; in this case, |c| c+1 runs iff h.maybe_encrypt succeeds
    let x = h.maybe_encrypt(ptxt).map(|c| c + 1);

    //"{:?}" is a debug format
    println!("{:?}", x);

    //this won't work, because you can't print Option<u32>
    //println!("x: {}", x);

    /* when you only care about one case (usually the commone case), compress "match" or "option" with if-let
     if let Some(c) = h.maybe_encrypt(ptxt) {
        println!("ciphertext: {}", c);

        if let Some(p) = h.maybe_decrypt(c){
            println!("plaintext: {}", p)
        }
     }
    */

    /* When you care about all cases, use match
        match h.maybe_encrypt(ptxt) {
            Some(c) => {
                println!("ciphertext: {}", c);
                match h.maybe_decrypt(c) {
                    Some(p) => println!("plaintext: {}", p),
                    None => println!("nothing decrypted"),
                }
            }
            None => println!("nothing encrypted"),
        }
    */

    /*A slightly modified version of the above, nested match-block */
    match h.maybe_encrypt(ptxt) {
        Some(c) => match h.maybe_decrypt(c) {
            Some(p) => println!("encryption and decryption success, plaintext: {}", p),
            None => println!("encryption success, decryption fail"),
        },
        None => println!("encryption fail"),
    }

    /*Using and_then construct
    and_then: Option<T>.and_then(f: T-> Option<U>)->Option<U>
    "If Option<T> is Some(x), run f(x); otherwise Option<T> is None, retain type None"
    in other words, Option = computation may fail, and .and_then(...) composes computations that may fail in a clean way, to avoid returning types like Option<Option<T>>
    */
    let result = h.maybe_encrypt(ptxt).and_then(|c| h.maybe_decrypt(c));
    println!("{:?}", result);

    let result = h.maybe_encrypt(ptxt).and_then(double_if_valid);
    println!("{:?}", result);

    println!("{:?}", process2(3));
    println!("{:?}", process2(0));
    println!("{:?}", process2(6));
}
*/
