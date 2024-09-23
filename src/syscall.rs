use alloc::vec::Vec;

pub fn debug(msg: &str) -> u64 {
    let mut data: Vec<u8> = msg.as_bytes().into();
    data.push(b'\0');
    ecall(data.as_ptr() as u64, 0, 0, 0, 0, 0, 0, 2177)
}

pub fn ecall(mut a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64) -> u64 {
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

pub fn exit(code: u64) -> ! {
    ecall(code, 0, 0, 0, 0, 0, 0, 93);
    loop {}
}

pub fn load_script_hash() -> [u8; 32] {
    let mut buf = [0; 32];
    let mut len: u64 = 32;
    ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, 0, 0, 0, 0, 2062);
    buf
}

pub fn load_script() -> crate::conversion::Script {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, 0, 0, 0, 0, 2052);
    assert!(len <= 32 * 1024);
    crate::conversion::Script::molecule_decode(&buf[..len as usize])
}

pub fn load_tx_hash() -> [u8; 32] {
    let mut buf = [0; 32];
    let mut len: u64 = 32;
    ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, 0, 0, 0, 0, 2061);
    buf
}

pub fn load_tx() -> crate::conversion::Transaction {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, 0, 0, 0, 0, 2051);
    assert!(len <= 32 * 1024);
    crate::conversion::Transaction::molecule_decode(&buf[..len as usize])
}
