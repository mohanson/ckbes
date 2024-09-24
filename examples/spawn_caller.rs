#![no_main]
#![no_std]

extern crate alloc;
extern crate ckbes;

#[no_mangle]
pub unsafe extern "C" fn main() -> u64 {
    let argv = ["Hello", " ", "World", "!"];
    let fds = ckbes::syscall::pipe();
    let pid = ckbes::syscall::spawn(1, ckbes::core::SOURCE_CELL_DEP, &argv, &[fds[1]]);
    let mut buf: [u8; 256] = [0; 256];
    let len = ckbes::syscall::read(fds[0], &mut buf);
    assert_eq!(len, 12);
    assert_eq!(&buf[..len as usize], b"Hello World!");
    ckbes::syscall::wait(pid);
    return 0;
}
