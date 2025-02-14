use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub static HEAP: spinning_top::Spinlock<[u8; 1024 * 1024]> = spinning_top::Spinlock::new([0; 1024 * 1024]);
#[global_allocator]
pub static LALC: linked_list_allocator::LockedHeap = linked_list_allocator::LockedHeap::empty();
pub static ARGS: spinning_top::Spinlock<Vec<String>> = spinning_top::Spinlock::new(Vec::new());

#[panic_handler]
pub fn panic_handler(i: &core::panic::PanicInfo) -> ! {
    // If the main thread panics it will terminate all your threads and end your program with code 101.
    // See: https://github.com/rust-lang/rust/blob/master/library/core/src/macros/panic.md
    crate::syscall::debug(&i.to_string());
    crate::syscall::exit(101)
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
        LALC.lock().init(HEAP.lock().as_mut_ptr(), 1024 * 1024);
    }
    for i in 0..argc {
        let argn = core::ffi::CStr::from_ptr(argv.add(i as usize).read());
        let argn = String::from(argn.to_string_lossy());
        ARGS.lock().push(argn);
    }
    core::arch::asm!("call main");
}
