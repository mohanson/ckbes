#![no_main]
#![no_std]

extern crate ckbes;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> u64 {
    let tx = ckbes::syscall::load_tx();
    let sighash_all = tx.hash_sighash_all();
    let witness_arg = ckbes::core::WitnessArgs::molecule_decode(&tx.witnesses[0]);
    assert_eq!(witness_arg.lock.unwrap(), sighash_all);
    return 0;
}
