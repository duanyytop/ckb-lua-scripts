use blake2b_ref::{Blake2b, Blake2bBuilder};

pub fn get_blake2b() -> Blake2b {
    Blake2bBuilder::new(32)
        .personal(b"ckb-default-hash")
        .build()
}

pub fn blake2b_256(message: &[u8]) -> [u8; 32] {
    let mut blake2b = get_blake2b();
    blake2b.update(&message);
    let mut result = [0; 32];
    blake2b.finalize(&mut result);
    result
}

pub fn blake2b_160(message: &[u8]) -> [u8; 20] {
    let result = blake2b_256(message);
    let mut hash = [0; 20];
    hash.copy_from_slice(&result[0..20]);
    hash
}