pub const DTB_MAGIC: u32 = 0xD00D_FEED;
pub const DTB_COMP_VERSION: u32 = 16;

pub const DTB_BEGIN_NODE: u32 = 1;
pub const DTB_END_NODE: u32 = 2;
pub const DTB_PROPERTY: u32 = 3;
pub const DTB_NOP: u32 = 4;
pub const DTB_END: u32 = 9;

#[repr(C)]
pub struct DtbHeader {
    pub magic: u32,
    pub total_size: u32,
    pub struct_offset: u32,
    pub strings_offset: u32,
    pub reserved_mem_offset: u32,
    pub version: u32,
    pub last_comp_version: u32,
    pub bsp_cpu_id: u32,
    pub strings_size: u32,
    pub struct_size: u32,
}

#[repr(C)]
pub struct DtbPropertyDesc {
    pub value_size: u32,
    pub name_offset: u32,
}
