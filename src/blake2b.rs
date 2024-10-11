pub fn blake2b_256<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
    let mut h = blake2b_ref::Blake2bBuilder::new(32).personal(b"ckb-default-hash").build();
    let mut r = [0; 32];
    h.update(data.as_ref());
    h.finalize(&mut r);
    r
}

pub fn blake2b_160<T: AsRef<[u8]>>(data: T) -> [u8; 20] {
    let h = blake2b_256(data);
    let mut r = [0; 20];
    r.copy_from_slice(&h[..20]);
    r
}
