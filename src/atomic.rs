//! Some Rust code can be compiled into atomic instructions for the RISC-V target. However, these atomic instructions
//! are not supported on ckb-vm. To address this issue, this module has been introduced.
//!
//! When the RISC-V atomic extension is disabled by specifying the `target-feature=-a` flag, LLVM will attempt to link
//! the atomic operations to functions prefixed with `__atomic` in this module. For more details, refer to the
//! [LLVM Atomics Documentation](https://llvm.org/docs/Atomics.html).
//!
//! On the CKB-VM, only a single thread is present, making dummy atomic operations sufficient for its purposes.

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_exchange_1(ptr: *mut u8, val: u8, _: isize) -> u8 {
    unsafe { ptr.replace(val) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_exchange_2(ptr: *mut u16, val: u16, _: isize) -> u16 {
    unsafe { ptr.replace(val) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_exchange_4(ptr: *mut u32, val: u32, _: isize) -> u32 {
    unsafe { ptr.replace(val) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_exchange_8(ptr: *mut u64, val: u64, _: isize) -> u64 {
    unsafe { ptr.replace(val) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_compare_exchange_1(
    ptr: *mut u8,
    expected: *mut u8,
    desired: u8,
    _: bool,
    _: isize,
    _: isize,
) -> bool {
    unsafe {
        let val = ptr.read();
        let ret = val == expected.read();
        if !ret {
            expected.write(val);
        } else {
            ptr.write(desired);
        }
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_compare_exchange_2(
    ptr: *mut u16,
    expected: *mut u16,
    desired: u16,
    _: bool,
    _: isize,
    _: isize,
) -> bool {
    unsafe {
        let val = ptr.read();
        let ret = val == expected.read();
        if !ret {
            expected.write(val);
        } else {
            ptr.write(desired);
        }
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_compare_exchange_4(
    ptr: *mut u32,
    expected: *mut u32,
    desired: u32,
    _: bool,
    _: isize,
    _: isize,
) -> bool {
    unsafe {
        let val = ptr.read();
        let ret = val == expected.read();
        if !ret {
            expected.write(val);
        } else {
            ptr.write(desired);
        }
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_compare_exchange_8(
    ptr: *mut u64,
    expected: *mut u64,
    desired: u64,
    _: bool,
    _: isize,
    _: isize,
) -> bool {
    unsafe {
        let val = ptr.read();
        let ret = val == expected.read();
        if !ret {
            expected.write(val);
        } else {
            ptr.write(desired);
        }
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_load_1(ptr: *const u8, _: isize) -> u8 {
    unsafe { ptr.read() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_load_2(ptr: *const u16, _: isize) -> u16 {
    unsafe { ptr.read() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_load_4(ptr: *const u32, _: isize) -> u32 {
    unsafe { ptr.read() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_load_8(ptr: *const u64, _: isize) -> u64 {
    unsafe { ptr.read() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_store_1(ptr: *mut u8, val: u8, _: isize) {
    unsafe { ptr.write(val) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_store_2(ptr: *mut u16, val: u16, _: isize) {
    unsafe { ptr.write(val) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_store_4(ptr: *mut u32, val: u32, _: isize) {
    unsafe { ptr.write(val) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_store_8(ptr: *mut u64, val: u64, _: isize) {
    unsafe { ptr.write(val) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_add_1(ptr: *mut u8, val: u8, _: isize) -> u8 {
    unsafe {
        let old = ptr.read();
        ptr.write(old.wrapping_add(val));
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_add_2(ptr: *mut u16, val: u16, _: isize) -> u16 {
    unsafe {
        let old = ptr.read();
        ptr.write(old.wrapping_add(val));
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_add_4(ptr: *mut u32, val: u32, _: isize) -> u32 {
    unsafe {
        let old = ptr.read();
        ptr.write(old.wrapping_add(val));
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_add_8(ptr: *mut u64, val: u64, _: isize) -> u64 {
    unsafe {
        let old = ptr.read();
        ptr.write(old.wrapping_add(val));
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_sub_1(ptr: *mut u8, val: u8, _: isize) -> u8 {
    unsafe {
        let old = ptr.read();
        ptr.write(old.wrapping_sub(val));
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_sub_2(ptr: *mut u16, val: u16, _: isize) -> u16 {
    unsafe {
        let old = ptr.read();
        ptr.write(old.wrapping_sub(val));
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_sub_4(ptr: *mut u32, val: u32, _: isize) -> u32 {
    unsafe {
        let old = ptr.read();
        ptr.write(old.wrapping_sub(val));
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_sub_8(ptr: *mut u64, val: u64, _: isize) -> u64 {
    unsafe {
        let old = ptr.read();
        ptr.write(old.wrapping_sub(val));
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_and_1(ptr: *mut u8, val: u8, _: isize) -> u8 {
    unsafe {
        let old = ptr.read();
        ptr.write(old & val);
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_and_2(ptr: *mut u16, val: u16, _: isize) -> u16 {
    unsafe {
        let old = ptr.read();
        ptr.write(old & val);
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_and_4(ptr: *mut u32, val: u32, _: isize) -> u32 {
    unsafe {
        let old = ptr.read();
        ptr.write(old & val);
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_and_8(ptr: *mut u64, val: u64, _: isize) -> u64 {
    unsafe {
        let old = ptr.read();
        ptr.write(old & val);
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_xor_1(ptr: *mut u8, val: u8, _: isize) -> u8 {
    unsafe {
        let old = ptr.read();
        ptr.write(old ^ val);
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_xor_2(ptr: *mut u16, val: u16, _: isize) -> u16 {
    unsafe {
        let old = ptr.read();
        ptr.write(old ^ val);
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_xor_4(ptr: *mut u32, val: u32, _: isize) -> u32 {
    unsafe {
        let old = ptr.read();
        ptr.write(old ^ val);
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_xor_8(ptr: *mut u64, val: u64, _: isize) -> u64 {
    unsafe {
        let old = ptr.read();
        ptr.write(old ^ val);
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_or_1(ptr: *mut u8, val: u8, _: isize) -> u8 {
    unsafe {
        let old = ptr.read();
        ptr.write(old | val);
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_or_2(ptr: *mut u16, val: u16, _: isize) -> u16 {
    unsafe {
        let old = ptr.read();
        ptr.write(old | val);
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_or_4(ptr: *mut u32, val: u32, _: isize) -> u32 {
    unsafe {
        let old = ptr.read();
        ptr.write(old | val);
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_or_8(ptr: *mut u64, val: u64, _: isize) -> u64 {
    unsafe {
        let old = ptr.read();
        ptr.write(old | val);
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_nand_1(ptr: *mut u8, val: u8, _: isize) -> u8 {
    unsafe {
        let old = ptr.read();
        ptr.write(!(old & val));
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_nand_2(ptr: *mut u16, val: u16, _: isize) -> u16 {
    unsafe {
        let old = ptr.read();
        ptr.write(!(old & val));
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_nand_4(ptr: *mut u32, val: u32, _: isize) -> u32 {
    unsafe {
        let old = ptr.read();
        ptr.write(!(old & val));
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_fetch_nand_8(ptr: *mut u64, val: u64, _: isize) -> u64 {
    unsafe {
        let old = ptr.read();
        ptr.write(!(old & val));
        old
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_add_fetch_1(ptr: *mut u8, val: u8, _: isize) -> u8 {
    unsafe {
        let old = ptr.read();
        let ret = old.wrapping_add(val);
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_add_fetch_2(ptr: *mut u16, val: u16, _: isize) -> u16 {
    unsafe {
        let old = ptr.read();
        let ret = old.wrapping_add(val);
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_add_fetch_4(ptr: *mut u32, val: u32, _: isize) -> u32 {
    unsafe {
        let old = ptr.read();
        let ret = old.wrapping_add(val);
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_add_fetch_8(ptr: *mut u64, val: u64, _: isize) -> u64 {
    unsafe {
        let old = ptr.read();
        let ret = old.wrapping_add(val);
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_sub_fetch_1(ptr: *mut u8, val: u8, _: isize) -> u8 {
    unsafe {
        let old = ptr.read();
        let ret = old.wrapping_sub(val);
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_sub_fetch_2(ptr: *mut u16, val: u16, _: isize) -> u16 {
    unsafe {
        let old = ptr.read();
        let ret = old.wrapping_sub(val);
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_sub_fetch_4(ptr: *mut u32, val: u32, _: isize) -> u32 {
    unsafe {
        let old = ptr.read();
        let ret = old.wrapping_sub(val);
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_sub_fetch_8(ptr: *mut u64, val: u64, _: isize) -> u64 {
    unsafe {
        let old = ptr.read();
        let ret = old.wrapping_sub(val);
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_and_fetch_1(ptr: *mut u8, val: u8, _: isize) -> u8 {
    unsafe {
        let old = ptr.read();
        let ret = old & val;
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_and_fetch_2(ptr: *mut u16, val: u16, _: isize) -> u16 {
    unsafe {
        let old = ptr.read();
        let ret = old & val;
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_and_fetch_4(ptr: *mut u32, val: u32, _: isize) -> u32 {
    unsafe {
        let old = ptr.read();
        let ret = old & val;
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_and_fetch_8(ptr: *mut u64, val: u64, _: isize) -> u64 {
    unsafe {
        let old = ptr.read();
        let ret = old & val;
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_xor_fetch_1(ptr: *mut u8, val: u8, _: isize) -> u8 {
    unsafe {
        let old = ptr.read();
        let ret = old ^ val;
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_xor_fetch_2(ptr: *mut u16, val: u16, _: isize) -> u16 {
    unsafe {
        let old = ptr.read();
        let ret = old ^ val;
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_xor_fetch_4(ptr: *mut u32, val: u32, _: isize) -> u32 {
    unsafe {
        let old = ptr.read();
        let ret = old ^ val;
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_xor_fetch_8(ptr: *mut u64, val: u64, _: isize) -> u64 {
    unsafe {
        let old = ptr.read();
        let ret = old ^ val;
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_or_fetch_1(ptr: *mut u8, val: u8, _: isize) -> u8 {
    unsafe {
        let old = ptr.read();
        let ret = old | val;
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_or_fetch_2(ptr: *mut u16, val: u16, _: isize) -> u16 {
    unsafe {
        let old = ptr.read();
        let ret = old | val;
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_or_fetch_4(ptr: *mut u32, val: u32, _: isize) -> u32 {
    unsafe {
        let old = ptr.read();
        let ret = old | val;
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_or_fetch_8(ptr: *mut u64, val: u64, _: isize) -> u64 {
    unsafe {
        let old = ptr.read();
        let ret = old | val;
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_nand_fetch_1(ptr: *mut u8, val: u8, _: isize) -> u8 {
    unsafe {
        let old = ptr.read();
        let ret = !(old & val);
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_nand_fetch_2(ptr: *mut u16, val: u16, _: isize) -> u16 {
    unsafe {
        let old = ptr.read();
        let ret = !(old & val);
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_nand_fetch_4(ptr: *mut u32, val: u32, _: isize) -> u32 {
    unsafe {
        let old = ptr.read();
        let ret = !(old & val);
        ptr.write(ret);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __atomic_nand_fetch_8(ptr: *mut u64, val: u64, _: isize) -> u64 {
    unsafe {
        let old = ptr.read();
        let ret = !(old & val);
        ptr.write(ret);
        ret
    }
}
