use core::mem::size_of;

use super::common::*;
use super::internal::*;
// use super::struct_item::*;

/// Reserved memory block.
#[derive(Debug)]
pub struct ReservedMem<'a> {
    buf: &'a mut [u8],
    offset: usize,
}

impl<'a> ReservedMem<'a> {
    /// Creates a new reserved memory block from a given buffer.
    pub fn from_buf(buf: &'a mut [u8]) -> Result<ReservedMem<'a>> {
        let buf = align_buf::<ReservedMemEntry>(buf)?;

        if buf.len() < size_of::<Header>() {
            return Err(Error::BufferTooSmall);
        }

        Ok(ReservedMem {
            buf,
            offset: size_of::<Header>(),
        })
    }

    /// Adds a new reserved memory entry.
    #[allow(clippy::cast_ptr_alignment)]
    pub fn add_entry(&mut self, address: u64, size: u64) -> Result<()> {
        if self.buf.len() < self.offset + size_of::<ReservedMemEntry>() {
            return Err(Error::BufferTooSmall);
        }

        let entry_be = unsafe {
            &mut *(self.buf.as_mut_ptr().add(self.offset)
                as *mut ReservedMemEntry)
        };

        entry_be.address = u64::to_be(address);
        entry_be.size = u64::to_be(size);

        self.offset += size_of::<ReservedMemEntry>();

        Ok(())
    }
}

/// Device tree blob writer.
#[derive(Debug)]
pub struct Writer<'a> {
    buf: &'a mut [u8],
    reserved_mem_offset: usize,
    struct_offset: usize,
    strings_offset: usize,
}

impl<'a> Writer<'a> {
    /// Creates a DTB writer from a given buffer.
    pub fn from_buf(buf: &'a mut [u8]) -> Result<Writer<'a>> {
        Writer::from_reserved_mem(ReservedMem::from_buf(buf)?)
    }

    /// Creates a DTB writer from a given reserved memory block.
    pub fn from_reserved_mem(
        mut reserved_mem: ReservedMem<'a>,
    ) -> Result<Writer<'a>> {
        reserved_mem.add_entry(0, 0)?;
        let len = reserved_mem.buf.len();
        Ok(Writer {
            buf: reserved_mem.buf,
            reserved_mem_offset: reserved_mem.offset,
            struct_offset: reserved_mem.offset,
            strings_offset: len,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER_U32_NUM: usize = size_of::<Header>() / size_of::<u32>();
    const ENTRY_U32_NUM: usize =
        size_of::<ReservedMemEntry>() / size_of::<u32>();

    fn assert_reserved_mem<'a, T>(func: fn(buf: &'a mut [u8]) -> Result<T>)
    where
        T: std::fmt::Debug,
    {
        aligned_buf!(tmp, [0u32; HEADER_U32_NUM + 1]);
        let len = tmp.len();
        let unaligned_buf = &mut tmp[1..len - 1];
        assert_eq!(func(unaligned_buf).unwrap_err(), Error::BufferTooSmall);

        aligned_buf!(buf, [0u32; HEADER_U32_NUM - 1]);
        assert_eq!(func(buf).unwrap_err(), Error::BufferTooSmall);
    }

    #[test]
    fn test_reserved_mem() {
        assert_reserved_mem(|buf| ReservedMem::from_buf(buf));

        aligned_buf!(buf, [0u32; HEADER_U32_NUM]);
        let mut reserved_mem = ReservedMem::from_buf(buf).unwrap();
        assert_eq!(
            reserved_mem.add_entry(1, 1).unwrap_err(),
            Error::BufferTooSmall
        );

        aligned_buf!(buf, [0u32; HEADER_U32_NUM + ENTRY_U32_NUM]);
        let mut reserved_mem = ReservedMem::from_buf(buf).unwrap();
        reserved_mem.add_entry(1, 1).unwrap();
        assert_eq!(
            reserved_mem.add_entry(1, 1).unwrap_err(),
            Error::BufferTooSmall
        );
    }

    #[test]
    fn test_new_writer() {
        assert_reserved_mem(|buf| Writer::from_buf(buf));

        aligned_buf!(buf, [0u32; HEADER_U32_NUM]);
        assert_eq!(Writer::from_buf(buf).unwrap_err(), Error::BufferTooSmall);

        let reserved_mem = ReservedMem::from_buf(buf).unwrap();
        assert_eq!(
            Writer::from_reserved_mem(reserved_mem).unwrap_err(),
            Error::BufferTooSmall
        );
    }
}
