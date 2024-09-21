#![no_main]
#![no_std]

extern crate ckbes;

#[no_mangle]
pub unsafe extern "C" fn main() -> u64 {
    let data = ckbes::ARGS.join(" ");
    ckbes::syscall_debug(data.as_ptr());
    return 0;
}