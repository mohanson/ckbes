pub fn blake2b_256(data: &[u8]) -> [u8; 32] {
    let mut h = blake2b_ref::Blake2bBuilder::new(32).personal(b"ckb-default-hash").build();
    let mut r = [0; 32];
    h.update(data);
    h.finalize(&mut r);
    r
}

pub fn blake2b_160(data: &[u8]) -> [u8; 20] {
    let h = blake2b_256(data);
    let mut r = [0; 20];
    r.copy_from_slice(&h[..20]);
    r
}
