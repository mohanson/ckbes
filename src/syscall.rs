use alloc::ffi::CString;
use alloc::vec::Vec;

pub fn current_cycles() -> u64 {
    return ecall(0, 0, 0, 0, 0, 0, 0, 2042);
}

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

pub fn exec(index: u64, source: u64, args: &[&str]) -> ! {
    let args_vec: Vec<u64> = args.iter().map(|e| CString::new(*e).unwrap().as_c_str().as_ptr() as u64).collect();
    let args_ptr = args_vec.as_ptr() as u64;
    let ret = ecall(index, source, 0, 0, args.len() as u64, args_ptr, 0, 2043);
    assert!(ret == 0);
    loop {}
}

pub fn exit(code: u64) -> ! {
    ecall(code, 0, 0, 0, 0, 0, 0, 93);
    loop {}
}

pub fn load_block_extension(index: u64, source: u64) -> Vec<u8> {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, index, source, 0, 0, 2104);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    buf[..len as usize].to_vec()
}

pub fn load_cell(index: u64, source: u64) -> crate::conversion::CellOutput {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, index, source, 0, 0, 2071);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    crate::conversion::CellOutput::molecule_decode(&buf[..len as usize])
}

pub fn load_cell_data(index: u64, source: u64) -> Vec<u8> {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, index, source, 0, 0, 2092);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    buf[..len as usize].to_vec()
}

pub fn load_header(index: u64, source: u64) -> crate::conversion::Header {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, index, source, 0, 0, 2072);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    crate::conversion::Header::molecule_decode(&buf[..len as usize])
}

pub fn load_input(index: u64, source: u64) -> crate::conversion::CellInput {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, index, source, 0, 0, 2073);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    crate::conversion::CellInput::molecule_decode(&buf[..len as usize])
}

pub fn load_script_hash() -> [u8; 32] {
    let mut buf = [0; 32];
    let mut len: u64 = 32;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, 0, 0, 0, 0, 2062);
    assert!(ret == 0);
    buf
}

pub fn load_script() -> crate::conversion::Script {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, 0, 0, 0, 0, 2052);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    crate::conversion::Script::molecule_decode(&buf[..len as usize])
}

pub fn load_tx_hash() -> [u8; 32] {
    let mut buf = [0; 32];
    let mut len: u64 = 32;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, 0, 0, 0, 0, 2061);
    assert!(ret == 0);
    buf
}

pub fn load_tx() -> crate::conversion::Transaction {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, 0, 0, 0, 0, 2051);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    crate::conversion::Transaction::molecule_decode(&buf[..len as usize])
}

pub fn vm_version() -> u64 {
    return ecall(0, 0, 0, 0, 0, 0, 0, 2041);
}

pub fn load_witness(index: u64, source: u64) -> Vec<u8> {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, index, source, 0, 0, 2074);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    buf[..len as usize].to_vec()
}
