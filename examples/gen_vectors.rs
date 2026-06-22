fn main() {
    let key = "test-key-12345";
    let enc = make_encrypt(key);
    let zeros = enc.zeros();
    println!("zeros: {}", hex::encode(&zeros));
    
    let stream_num: u64 = 0x100000001;
    let offset: u64 = 0;
    let data = b"hello world";
    let ct = enc.segment(stream_num, offset, data);
    println!("ciphertext: {}", hex::encode(&ct));
    
    // verify roundtrip
    let pt = enc.segment(stream_num, offset, &ct);
    assert_eq!(&pt, data);
    println!("roundtrip OK");
}

use aes::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
type Aes128Ctr64BE = ctr::Ctr64BE<aes::Aes128>;
const SALT: &str = "This is a non-random salt for sshx.io, since we want to stretch the security of 83-bit keys!";

struct Encrypt { aes_key: [u8; 16] }
impl Encrypt {
    fn new(key: &str) -> Self {
        use argon2::{Algorithm, Argon2, Params, Version};
        let hasher = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::new(19 * 1024, 2, 1, Some(16)).unwrap());
        let mut aes_key = [0; 16];
        hasher.hash_password_into(key.as_bytes(), SALT.as_bytes(), &mut aes_key).unwrap();
        Self { aes_key }
    }
    fn zeros(&self) -> Vec<u8> {
        let mut zeros = [0; 16];
        let mut cipher = Aes128Ctr64BE::new(&self.aes_key.into(), &zeros.into());
        cipher.apply_keystream(&mut zeros);
        zeros.to_vec()
    }
    fn segment(&self, stream_num: u64, offset: u64, data: &[u8]) -> Vec<u8> {
        let mut iv = [0; 16];
        iv[0..8].copy_from_slice(&stream_num.to_be_bytes());
        let mut cipher = Aes128Ctr64BE::new(&self.aes_key.into(), &iv.into());
        let mut buf = data.to_vec();
        cipher.seek(offset);
        cipher.apply_keystream(&mut buf);
        buf
    }
}
fn make_encrypt(key: &str) -> Encrypt { Encrypt::new(key) }
