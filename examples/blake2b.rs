#![no_main]
#![no_std]

extern crate ckbes;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> u64 {
    let pubkey = [
        0x02, 0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0, 0x62, 0x95, 0xce, 0x87, 0x0b, 0x07, 0x02,
        0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81, 0x5b, 0x16, 0xf8, 0x17, 0x98,
    ];
    let expect = [
        0x75, 0x17, 0x8f, 0x34, 0x54, 0x9c, 0x5f, 0xe9, 0xcd, 0x1a, 0x0c, 0x57, 0xae, 0xbd, 0x01, 0xe7, 0xdd, 0xf9,
        0x24, 0x9e,
    ];
    let result = ckbes::blake2b::blake2b_160(&pubkey);
    assert_eq!(expect, result);
    return 0;
}
