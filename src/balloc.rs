//! A hand written buddy allocator implementation for no_std environments.
//!
//! This module implements a buddy memory allocator that manages a fixed-size memory pool.
//! The buddy allocator works by recursively splitting memory blocks into pairs of equal-sized "buddies" until it finds
//! a block of the appropriate size. When memory is freed, it attempts to merge adjacent buddy blocks back together to
//! reduce fragmentation.

use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr::{self, addr_of_mut};

pub static MIN_BLOCK: usize = 64;
pub static MAX_ORDER: usize = 14;
pub static MAX_TOTAL: usize = 1024 * 1024;
pub static mut FREE_LIST: [usize; MAX_ORDER + 1] = {
    let mut list = [usize::MAX; MAX_ORDER + 1];
    list[MAX_ORDER] = 0;
    list
};
pub static mut PRE_ALLOC: [u8; MAX_TOTAL] = {
    let mut alloc = [0; MAX_TOTAL];
    alloc[0] = 0xff;
    alloc[1] = 0xff;
    alloc[2] = 0xff;
    alloc[3] = 0xff;
    alloc[4] = 0xff;
    alloc[5] = 0xff;
    alloc[6] = 0xff;
    alloc[7] = 0xff;
    alloc
};

pub struct Algorithm;

impl Algorithm {
    pub fn alloc_block(&mut self, order: usize) -> *mut u8 {
        unsafe {
            if FREE_LIST[order] != usize::MAX {
                let block_offset = FREE_LIST[order];
                let block_ptr = (addr_of_mut!(PRE_ALLOC) as *mut u8).add(block_offset);
                FREE_LIST[order] = *(block_ptr as *const usize);
                return block_ptr;
            }
            if order >= MAX_ORDER {
                return ptr::null_mut();
            }
            let block = self.alloc_block(order + 1);
            if block.is_null() {
                return ptr::null_mut();
            }
            let block_size = MIN_BLOCK << order;
            let buddy_ptr = block.add(block_size);
            let buddy_offset = buddy_ptr.offset_from(addr_of_mut!(PRE_ALLOC) as *const u8) as usize;
            *(buddy_ptr as *mut usize) = usize::MAX;
            FREE_LIST[order] = buddy_offset;
            block
        }
    }

    pub fn close_block(&mut self, order: usize, ptr: *mut u8) {
        unsafe {
            if ptr.is_null() {
                return;
            }
            let start_ptr = addr_of_mut!(PRE_ALLOC) as *const u8;
            let block_ptr = ptr;
            let block_size = MIN_BLOCK << order;
            let block_offset = block_ptr.offset_from(start_ptr) as usize;
            let block_idx = block_offset / block_size;
            let buddy_idx = block_idx ^ 1;
            let buddy_offset = buddy_idx * block_size;
            let buddy_ptr = start_ptr.add(buddy_offset) as *mut u8;
            if self.buddy_unused(order, buddy_ptr) {
                self.buddy_close(order, buddy_ptr);
                self.close_block(order + 1, core::cmp::min(block_ptr, buddy_ptr));
                return;
            }
            *(block_ptr as *mut usize) = FREE_LIST[order];
            FREE_LIST[order] = block_offset;
        }
    }

    pub fn alloc(&mut self, layout: Layout) -> *mut u8 {
        self.alloc_block(log2(MIN_BLOCK, clp2(layout.size()).max(MIN_BLOCK)))
    }

    pub fn close(&mut self, layout: Layout, ptr: *mut u8) {
        self.close_block(log2(MIN_BLOCK, clp2(layout.size()).max(MIN_BLOCK)), ptr);
    }

    fn buddy_unused(&mut self, order: usize, ptr: *mut u8) -> bool {
        unsafe {
            let base = addr_of_mut!(PRE_ALLOC) as *const u8;
            let need = ptr.offset_from(addr_of_mut!(PRE_ALLOC) as *const u8) as usize;
            let mut prev = FREE_LIST[order];
            loop {
                if prev == need {
                    break prev != usize::MAX;
                }
                if prev == usize::MAX {
                    break prev == need;
                }
                let next = *(base.add(prev) as *const usize);
                prev = next;
            }
        }
    }

    fn buddy_close(&mut self, order: usize, ptr: *mut u8) {
        unsafe {
            let base = addr_of_mut!(PRE_ALLOC) as *const u8;
            let need = ptr.offset_from(addr_of_mut!(PRE_ALLOC) as *const u8) as usize;
            let mut prev = FREE_LIST[order];
            if prev == need {
                FREE_LIST[order] = *(base.add(need) as *const usize);
                return;
            }
            loop {
                if prev == usize::MAX {
                    break;
                }
                let next = *(base.add(prev) as *const usize);
                if next == need {
                    *(base.add(prev) as *mut usize) = *(base.add(next) as *const usize);
                    break;
                }
                prev = next;
            }
        }
    }
}

pub struct Allocator {
    inner: UnsafeCell<Algorithm>,
}

impl Allocator {
    pub const fn global() -> Self {
        Allocator { inner: UnsafeCell::new(Algorithm {}) }
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { (*self.inner.get()).alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { (*self.inner.get()).close(layout, ptr) }
    }
}

unsafe impl Sync for Allocator {}

fn clp2(n: usize) -> usize {
    n.next_power_of_two()
}

fn log2(m: usize, n: usize) -> usize {
    let mut m = m;
    for i in 0..64 {
        if m == n {
            return i;
        }
        m <<= 1;
    }
    unreachable!()
}
