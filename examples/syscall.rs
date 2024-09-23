#![no_main]
#![no_std]

extern crate alloc;
extern crate ckbes;
use alloc::format;

#[no_mangle]
pub unsafe extern "C" fn main() -> u64 {
    let txid = ckbes::syscall::load_tx_hash();
    ckbes::syscall::debug(&hex::encode(txid));

    let tx = ckbes::syscall::load_tx();
    ckbes::syscall::debug(&format!("{:?}", tx));
    return 0;
}
