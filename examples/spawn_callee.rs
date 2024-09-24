#![no_main]
#![no_std]

extern crate ckbes;

#[no_mangle]
pub unsafe extern "C" fn main() -> u64 {
    let fds = ckbes::syscall::inherited_fds();
    let out = ckbes::global::ARGS.join("");
    ckbes::syscall::write(fds[0], out.as_bytes());
    ckbes::syscall::close(fds[0]);
    return 0;
}
