#![no_main]
#![no_std]

extern crate alloc;
extern crate ckbes;
use alloc::format;

#[no_mangle]
pub unsafe extern "C" fn main() -> u64 {
    let cell = ckbes::syscall::load_cell(0, ckbes::conversion::SOURCE_INPUT);
    ckbes::syscall::debug(&format!("{:?}", cell));

    let cell_data = ckbes::syscall::load_cell_data(0, ckbes::conversion::SOURCE_INPUT);
    ckbes::syscall::debug(&format!("{:?}", cell_data));

    let input = ckbes::syscall::load_input(0, ckbes::conversion::SOURCE_INPUT);
    ckbes::syscall::debug(&format!("{:?}", input));

    let script_hash = ckbes::syscall::load_script_hash();
    ckbes::syscall::debug(&format!("{:?}", script_hash));

    let script = ckbes::syscall::load_script();
    ckbes::syscall::debug(&format!("{:?}", script));

    let tx_hash = ckbes::syscall::load_tx_hash();
    ckbes::syscall::debug(&format!("{:?}", tx_hash));

    let tx = ckbes::syscall::load_tx();
    ckbes::syscall::debug(&format!("{:?}", tx));

    return 0;
}
