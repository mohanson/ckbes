#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub static mut HEAPS: [u8; 1024 * 1024] = [0; 1024 * 1024];
#[global_allocator]
pub static ALLOC: linked_list_allocator::LockedHeap = linked_list_allocator::LockedHeap::empty();
pub static mut ARGS: Vec<String> = Vec::new();

#[panic_handler]
pub fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    // If the main thread panics it will terminate all your threads and end your program with code 101.
    // See: https://github.com/rust-lang/rust/blob/master/library/core/src/macros/panic.md
    syscall_exit(101)
}

pub fn syscall(mut a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64) -> u64 {
    unsafe {
        core::arch::asm!(
          "ecall",
          inout("a0") a0,
          in("a1") a1,
          in("a2") a2,
          in("a3") a3,
          in("a4") a4,
          in("a5") a5,
          in("a6") a6,
          in("a7") a7
        )
    }
    a0
}

pub fn syscall_debug(buf: *const u8) -> u64 {
    syscall(buf as u64, 0, 0, 0, 0, 0, 0, 2177)
}

pub fn syscall_exit(code: u64) -> ! {
    syscall(code, 0, 0, 0, 0, 0, 0, 93);
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn _start() {
    core::arch::asm!(
        "lw a0,0(sp)", // Argc.
        "add a1,sp,8", // Argv.
        "li a2,0",     // Envp.
        "call _entry",
        "li a7, 93",
        "ecall",
    );
}

#[no_mangle]
pub unsafe extern "C" fn _entry(argc: u64, argv: *const *const i8) {
    unsafe {
        ALLOC.lock().init(HEAPS.as_mut_ptr(), 1024 * 1024);
    }
    for i in 0..argc {
        let argn = core::ffi::CStr::from_ptr(argv.add(i as usize).read());
        let argn = String::from(argn.to_string_lossy());
        ARGS.push(argn);
    }
    core::arch::asm!("call main");
}
