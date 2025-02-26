/// Generates a 256-bit (32-byte) blake2b hash from the input data.
pub fn blake2b_256<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
    let mut p = blake2ya::blake2b_params();
    p.digest(32);
    p.person(b"ckb-default-hash");
    let mut h = blake2ya::blake2b(p);
    h.update(data.as_ref());
    let mut r = [0; 32];
    h.digest(&mut r);
    r
}

/// Generates a 160-bit (20-byte) blake2b hash by truncating the output of blake2b_256.
pub fn blake2b_160<T: AsRef<[u8]>>(data: T) -> [u8; 20] {
    let h = blake2b_256(data);
    let mut r = [0; 20];
    r.copy_from_slice(&h[..20]);
    r
}
