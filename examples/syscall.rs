#![no_main]
#![no_std]

extern crate alloc;
extern crate ckbes;
use alloc::format;

#[no_mangle]
pub unsafe extern "C" fn main() -> u64 {
    let script_hash = ckbes::syscall::load_script_hash();
    ckbes::syscall::debug(&hex::encode(script_hash));

    let script = ckbes::syscall::load_script();
    ckbes::syscall::debug(&format!("{:?}", script));

    let tx_hash = ckbes::syscall::load_tx_hash();
    ckbes::syscall::debug(&hex::encode(tx_hash));

    let tx = ckbes::syscall::load_tx();
    ckbes::syscall::debug(&format!("{:?}", tx));

    return 0;
}
