use crate::balloc::Allocator;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[global_allocator]
pub static LALC: Allocator = Allocator::global();
pub static mut ARGS: Vec<String> = Vec::new();

#[panic_handler]
pub fn panic_handler(i: &core::panic::PanicInfo) -> ! {
    // If the main thread panics it will terminate all your threads and end your program with code 101.
    // See: https://github.com/rust-lang/rust/blob/master/library/core/src/macros/panic.md
    crate::syscall::debug(&i.to_string());
    crate::syscall::exit(101)
}

#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() {
    unsafe {
        core::arch::asm!(
            "lw a0,0(sp)", // Argc.
            "add a1,sp,8", // Argv.
            "li a2,0",     // Envp.
            "call _entry",
            "li a7, 93",
            "ecall",
        );
    }
}

#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _entry(argc: u64, argv: *const *const u8) {
    unsafe {
        for i in 0..argc {
            let argn = core::ffi::CStr::from_ptr(argv.add(i as usize).read());
            let argn = String::from(argn.to_string_lossy());
            #[allow(static_mut_refs)]
            ARGS.push(argn);
        }
        core::arch::asm!("call main");
    }
}
