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

To run a test sample execute:
```sh
cargo run --example dump src/test_dtb/sample.dtb
```
