# CKB Easy Script

CKB Easy Script is a framework for developing ckb contracts. It is currently in early development stages. You can see how to use ckbes in the examples. If you have installed [ckb-debugger](https://github.com/nervosnetwork/ckb-standalone-debugger), you can execute the contract script in the virtual environment by:

```sh
$ cargo run --example echo Hello World!
# Script log: Hello World!
# Run result: 0
# All cycles: 36184(35.3K)

$ cargo run --example exit_0
# Run result: 0
# All cycles: 9244(9.0K)

$ cargo run --example exit_1
# Run result: 0
# All cycles: 9244(9.0K)

$ cargo run --example syscall
# Script log: 20903
# Script log: CellOutput { capacity: 335240900000000, lock: Script { code_hash: [181, 52, 154, 110, 109, 64, 89, 129, 126, 137, 50, 134, 49, 226, 182, 227, 9, 209, 201, 146, 54, 66, 229, 7, 151, 192, 147, 43, 50, 168, 197, 107], hash_type: 1, args: [] }, kype: None }
# Script log: []
# Script log: CellInput { since: 0, previous_output: OutPoint { tx_hash: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], index: 1 } }
# Script log: [99, 98, 129, 39, 198, 215, 197, 23, 85, 208, 142, 190, 243, 143, 89, 124, 49, 224, 154, 244, 217, 5, 99, 107, 101, 53, 67, 28, 27, 39, 238, 181]
# Script log: Script { code_hash: [181, 52, 154, 110, 109, 64, 89, 129, 126, 137, 50, 134, 49, 226, 182, 227, 9, 209, 201, 146, 54, 66, 229, 7, 151, 192, 147, 43, 50, 168, 197, 107], hash_type: 1, args: [] }
# Script log: [142, 83, 54, 21, 225, 34, 144, 140, 95, 27, 33, 97, 65, 225, 79, 145, 130, 184, 126, 130, 231, 228, 117, 112, 202, 194, 140, 161, 25, 32, 110, 167]
# Script log: Transaction { raw: RawTransaction { version: 0, cell_deps: [CellDep { out_point: OutPoint { tx_hash: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], index: 0 }, dep_type: 0 }], header_deps: [], inputs: [CellInput { since: 0, previous_output: OutPoint { tx_hash: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], index: 1 } }], outputs: [CellOutput { capacity: 0, lock: Script { code_hash: [181, 52, 154, 110, 109, 64, 89, 129, 126, 137, 50, 134, 49, 226, 182, 227, 9, 209, 201, 146, 54, 66, 229, 7, 151, 192, 147, 43, 50, 168, 197, 107], hash_type: 1, args: [] }, kype: None }], outputs_data: [] }, witnesses: [[]] }
# Script log: 2
# Run result: 0
# All cycles: 1522164(1.5M)
```

# Licences

MIT.
