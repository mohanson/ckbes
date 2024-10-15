#![no_main]
#![no_std]

extern crate ckbes;

#[no_mangle]
pub unsafe extern "C" fn main() -> u64 {
    let argv = ["Hello", "World!"];
    let fds = ckbes::syscall::pipe();
    let pid = ckbes::syscall::spawn(1, ckbes::core::SOURCE_CELL_DEP, &argv, &[fds[1]]);
    let data = ckbes::syscall::read_all(fds[0]);
    assert_eq!(&data, b"Hello World!");
    ckbes::syscall::wait(pid);
    return 0;
}
