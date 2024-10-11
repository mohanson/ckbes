use alloc::ffi::CString;
use alloc::vec::Vec;

pub fn close(fd: u64) {
    let ret = ecall(fd, 0, 0, 0, 0, 0, 0, 2608);
    assert!(ret == 0);
}

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

pub fn inherited_fds() -> Vec<u64> {
    let mut buf = [0; 32];
    let mut len: u64 = 32;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, 0, 0, 0, 0, 2607);
    assert!(ret == 0);
    buf[..len as usize].to_vec()
}

pub fn load_block_extension(index: u64, source: u64) -> Vec<u8> {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, index, source, 0, 0, 2104);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    buf[..len as usize].to_vec()
}

pub fn load_cell(index: u64, source: u64) -> crate::core::CellOutput {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, index, source, 0, 0, 2071);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    crate::core::CellOutput::molecule_decode(&buf[..len as usize])
}

pub fn load_cell_by_field(index: u64, source: u64, field: u64) -> Vec<u8> {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, index, source, field, 0, 2081);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    buf[..len as usize].to_vec()
}

pub fn load_cell_data(index: u64, source: u64) -> Vec<u8> {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, index, source, 0, 0, 2092);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    buf[..len as usize].to_vec()
}

pub fn load_header(index: u64, source: u64) -> crate::core::Header {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, index, source, 0, 0, 2072);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    crate::core::Header::molecule_decode(&buf[..len as usize])
}

pub fn load_header_by_field(index: u64, source: u64, field: u64) -> Vec<u8> {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, index, source, field, 0, 2082);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    buf[..len as usize].to_vec()
}

pub fn load_input(index: u64, source: u64) -> crate::core::CellInput {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, index, source, 0, 0, 2073);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    crate::core::CellInput::molecule_decode(&buf[..len as usize])
}

pub fn load_input_by_field(index: u64, source: u64, field: u64) -> Vec<u8> {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, index, source, field, 0, 2083);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    buf[..len as usize].to_vec()
}

pub fn load_script_hash() -> [u8; 32] {
    let mut buf = [0; 32];
    let mut len: u64 = 32;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, 0, 0, 0, 0, 2062);
    assert!(ret == 0);
    buf
}

pub fn load_script() -> crate::core::Script {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, 0, 0, 0, 0, 2052);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    crate::core::Script::molecule_decode(&buf[..len as usize])
}

pub fn load_tx_hash() -> [u8; 32] {
    let mut buf = [0; 32];
    let mut len: u64 = 32;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, 0, 0, 0, 0, 2061);
    assert!(ret == 0);
    buf
}

pub fn load_tx() -> crate::core::Transaction {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, 0, 0, 0, 0, 2051);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    crate::core::Transaction::molecule_decode(&buf[..len as usize])
}

pub fn load_witness(index: u64, source: u64) -> Vec<u8> {
    let mut buf = [0; 32 * 1024];
    let mut len: u64 = 32 * 1024;
    let ret = ecall(buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, index, source, 0, 0, 2074);
    assert!(ret == 0);
    assert!(len <= 32 * 1024);
    buf[..len as usize].to_vec()
}

pub fn pipe() -> [u64; 2] {
    let mut fds: [u64; 2] = [0, 0];
    let ret = ecall(fds.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, 2604);
    assert!(ret == 0);
    fds
}

pub fn process_id() -> u64 {
    ecall(0, 0, 0, 0, 0, 0, 0, 2603)
}

pub fn read(fd: u64, buf: &mut [u8]) -> u64 {
    let mut len: u64 = buf.len() as u64;
    let ret = ecall(fd, buf.as_mut_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, 0, 0, 0, 2606);
    assert!(ret == 0);
    len
}

pub fn spawn(index: u64, source: u64, args: &[&str], fds: &[u64]) -> u64 {
    let args_vec: Vec<CString> = args.iter().map(|e| CString::new(*e).unwrap()).collect();
    let args_vec: Vec<u64> = args_vec.iter().map(|e| e.as_bytes_with_nul().as_ptr() as u64).collect();
    let args_ptr = args_vec.as_ptr() as u64;
    let mut fdr = Vec::new();
    fdr.extend_from_slice(fds);
    fdr.push(0);
    let fdr_ptr = fdr.as_ptr() as u64;
    let mut pid: u64 = 0;
    let spgs = [args.len() as u64, args_ptr, core::ptr::addr_of_mut!(pid) as u64, fdr_ptr];
    let spgs_ptr = spgs.as_ptr() as u64;
    let ret = ecall(index, source, 0, 0, spgs_ptr, 0, 0, 2601);
    assert!(ret == 0);
    pid
}

pub fn vm_version() -> u64 {
    return ecall(0, 0, 0, 0, 0, 0, 0, 2041);
}

pub fn wait(pid: u64) -> u64 {
    let mut code: u64 = 0;
    let ret = ecall(pid, core::ptr::addr_of_mut!(code) as u64, 0, 0, 0, 0, 0, 2602);
    assert!(ret == 0);
    code
}

pub fn write(fd: u64, buf: &[u8]) -> u64 {
    let mut len: u64 = buf.len() as u64;
    let ret = ecall(fd, buf.as_ptr() as u64, core::ptr::addr_of_mut!(len) as u64, 0, 0, 0, 0, 2605);
    assert!(ret == 0);
    len
}
