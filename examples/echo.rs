#![no_main]
#![no_std]

extern crate ckbes;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> u64 {
    #[allow(static_mut_refs)]
    let data = unsafe { ckbes::global::ARGS.join(" ") };
    ckbes::syscall::debug(&data);
    return 0;
}
