# DTB
## Device tree blob utilities

This `no_std` crate contains types for reading and writing DTBs. Here is a
code showing how to read a DTB-file:

```rust
let mut buf = Vec::new();
let mut file = File::open("example.dtb").unwrap();
file.read_to_end(&mut buf).unwrap();
let reader = Reader::read(buf.as_slice()).unwrap();

for entry in reader.reserved_mem_entries() {
    println!("reserved: {:?}, {:?}", entry.address, entry.size);
}

let root = reader.struct_items();
let (prop, _) =
    root.path_struct_items("/node/property").next().unwrap();
println!("property: {:?}, {:?}", prop.name(), prop.value_str());

let (node, node_iter) =
    root.path_struct_items("/node/node2").next().unwrap();
println!("node: {:?}@{:?}", node.node_name(), node.unit_address());

let mut buf = [0; 32];

let (prop, _) = node_iter.path_struct_items("property").next().unwrap();
println!(
    "property: {:?}, {:?}",
    prop.name(),
    prop.value_str_list(&mut buf)
);

let (prop, _) =
    node_iter.path_struct_items("property2").next().unwrap();
println!(
    "property: {:?}, {:?}",
    prop.name(),
    prop.value_u32_list(&mut buf)
);
```

To read DTB directly from a memory address use `Reader::read_from_address()`.

To run a test sample execute:
```sh
cargo run --example dump src/test_dtb/sample.dtb
```

## Fuzzing instructions

The reader (and methods of read items) can be fuzzed with
[`cargo-fuzz`/`libfuzzer`] which can be installed as `cargo install
cargo-fuzz`. Note that the coverage is not yet complete but provides a
straightforward harness.  The baseline corpus is the directory of tests is
`src/test_dtb`. Note that this command will require a nightly compiler.

```
cargo fuzz run reader src/test_dtb
```

[`cargo-fuzz`/`libfuzzer`]: https://github.com/rust-fuzz/cargo-fuzz
