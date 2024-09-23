#![no_main]
#![no_std]

extern crate ckbes;

#[no_mangle]
pub unsafe extern "C" fn main() -> u64 {
    let txid = ckbes::syscall::load_tx_hash();
    ckbes::syscall::debug(&hex::encode(txid));
    return 0;
}
