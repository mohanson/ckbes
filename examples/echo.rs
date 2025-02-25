#![no_main]
#![no_std]

extern crate ckbes;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> u64 {
    let data = ckbes::global::ARGS.lock().join(" ");
    ckbes::syscall::debug(&data);
    return 0;
}
