extern crate dtb;

use dtb::{Reader, StructItem};
use std::fs;
use std::io::Read;

fn main() {
    let mut buf = Vec::new();
    let mut file = fs::File::open(
        std::env::args()
            .nth(1)
            .expect("Need path to DTB file as argument"),
    )
    .unwrap();
    file.read_to_end(&mut buf).unwrap();
    let reader = Reader::read(buf.as_slice()).unwrap();

    for entry in reader.reserved_mem_entries() {
        println!("reserved: {:?} bytes at {:?}", entry.size, entry.address);
    }

    let mut indent = 0;
    for entry in reader.struct_items() {
        match entry {
            StructItem::BeginNode { name } => {
                println!("{:indent$}{} {{", "", name, indent = indent);
                indent += 2;
            }
            StructItem::EndNode => {
                indent -= 2;
                println!("{:indent$}}}", "", indent = indent);
            }
            StructItem::Property { name, value } => {
                println!("{:indent$}{}: {:?}", "", name, value, indent = indent)
            }
        }
    }
}
