pub const DTB_MAGIC: u32 = 0xD00D_FEED;
pub const COMP_VERSION: u32 = 16;

pub const TOK_BEGIN_NODE: u32 = 1;
pub const TOK_END_NODE: u32 = 2;
pub const TOK_PROPERTY: u32 = 3;
pub const TOK_NOP: u32 = 4;
pub const TOK_END: u32 = 9;

#[repr(C)]
pub struct Header {
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
pub struct PropertyDesc {
    pub value_size: u32,
    pub name_offset: u32,
}
