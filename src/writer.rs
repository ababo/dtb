use core::mem::size_of;

use super::common::*;
use super::internal::*;
// use super::struct_item::*;

/// Reserved memory block.
#[derive(Debug)]
pub struct ReservedMemBlock<'a> {
    buf: &'a mut [u8],
    offset: usize,
}

impl<'a> ReservedMemBlock<'a> {
    /// Creates a new reserved memory block from a given buffer.
    pub fn from_buf(buf: &'a mut [u8]) -> Result<ReservedMemBlock<'a>> {
        let buf = align_buf::<ReservedMemEntry>(buf)?;

        if buf.len() < size_of::<Header>() {
            return Err(Error::BufferTooSmall);
        }

        Ok(ReservedMemBlock {
            buf,
            offset: size_of::<Header>(),
        })
    }

    /// Adds a new reserved memory entry.
    #[allow(clippy::cast_ptr_alignment)]
    pub fn add_entry(&mut self, entry: &ReservedMemEntry) -> Result<()> {
        if self.buf.len() < self.offset + size_of::<ReservedMemEntry>() {
            return Err(Error::BufferTooSmall);
        }

        let entry_be = unsafe {
            &mut *(self.buf.as_mut_ptr().add(self.offset)
                as *mut ReservedMemEntry)
        };

        entry_be.address = u64::to_be(entry.address);
        entry_be.size = u64::to_be(entry.size);

        self.offset += size_of::<ReservedMemEntry>();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER_U32_NUM: usize = size_of::<Header>() / size_of::<u32>();
    const ENTRY_U32_NUM: usize =
        size_of::<ReservedMemEntry>() / size_of::<u32>();

    #[test]
    fn test_reserved_mem() {
        aligned_buf!(tmp, [0u32; HEADER_U32_NUM + 1]);
        let len = tmp.len();
        let mut unaligned_buf = &mut tmp[1..len - 1];
        assert_eq!(
            ReservedMemBlock::from_buf(&mut unaligned_buf).unwrap_err(),
            Error::BufferTooSmall
        );

        aligned_buf!(buf, [0u32; HEADER_U32_NUM - 1]);
        assert_eq!(
            ReservedMemBlock::from_buf(&mut buf).unwrap_err(),
            Error::BufferTooSmall
        );

        let entry = ReservedMemEntry {
            address: 0x1000,
            size: 0x100,
        };

        aligned_buf!(buf, [0u32; HEADER_U32_NUM]);
        let mut reserved_mem = ReservedMemBlock::from_buf(&mut buf).unwrap();
        assert_eq!(
            reserved_mem.add_entry(&entry).unwrap_err(),
            Error::BufferTooSmall
        );

        aligned_buf!(buf, [0u32; HEADER_U32_NUM + ENTRY_U32_NUM]);
        let mut reserved_mem = ReservedMemBlock::from_buf(&mut buf).unwrap();
        reserved_mem.add_entry(&entry).unwrap();
        assert_eq!(
            reserved_mem.add_entry(&entry).unwrap_err(),
            Error::BufferTooSmall
        );
    }
}
