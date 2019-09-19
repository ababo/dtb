#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate dtb;

fuzz_target!(|data: &[u8]| {
    let mut large_buffer = vec![0; 1024*1024];
    let reader = match dtb::Reader::read(data) {
        Ok(reader) => reader,
        Err(_) => return,
    };

    for item in reader.struct_items() {
        let _ = item.is_begin_node();
        let _ = item.is_property();
        let _ = item.name();
        let _ = item.node_name();
        let _ = item.unit_address();
        let _ = item.value();
        let _ = item.value_str();
        let _ = item.value_str_list(&mut large_buffer[..]);
        let _ = item.value_u32_list(&mut large_buffer[..]);
    }

    for entry in reader.reserved_mem_entries() {
        drop(entry);
    }
});
