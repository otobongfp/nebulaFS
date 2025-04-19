use std::io::{Read, Seek, SeekFrom, Write};
use std::mem::size_of;
use bytemuck::{Pod, Zeroable};
use crate::block::{BlockDevice, BLOCK_SIZE};

#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
pub struct Superblock {
    pub total_blocks: u64,
    pub free_blocks: u64,
    pub inode_start: u64,
    pub inode_count: u64,
    pub bitmap_start: u64,
    pub data_start: u64,
    pub magic: u32,
    pub _padding: u32,
}
// pub struct Superblock {
//     pub magic: u32,
//     pub total_blocks: u64,
//     pub free_blocks: u64,
//     pub inode_start: u64,
//     pub inode_count: u64,
//     pub bitmap_start: u64,
//     pub data_start: u64,
// }

impl Superblock {
    pub const MAGIC: u32 = 0xDEADC0DE;

    pub fn new(total_blocks: u64, inode_count: u64) -> Self {
        let inode_blocks = (inode_count * 128 + (BLOCK_SIZE as u64 - 1)) / BLOCK_SIZE as u64;
        let bitmap_blocks = (total_blocks + 7) / 8 / BLOCK_SIZE as u64;

        let inode_start = 1;
        let bitmap_start = inode_start + inode_blocks;
        let data_start = bitmap_start + bitmap_blocks;

        Self {
            total_blocks,
            free_blocks: total_blocks - data_start,
            inode_start,
            inode_count,
            bitmap_start,
            data_start,
            magic: Self::MAGIC,
            _padding:0
        }
    }

    pub fn write_to(&self, device: &mut BlockDevice) -> std::io::Result<()> {
        let mut buffer = [0u8; BLOCK_SIZE];
        let bytes = bytemuck::bytes_of(self);
        buffer[..bytes.len()].copy_from_slice(bytes);
        device.write_block(0, &buffer)?;
        Ok(())
    }

    pub fn read_from(device: &mut BlockDevice) -> std::io::Result<Self> {
        let buffer = device.read_block(0)?;
        let sb: Superblock = *bytemuck::from_bytes(&buffer[..size_of::<Self>()]);
        if sb.magic != Self::MAGIC {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid superblock magic number",
            ));
        }
        Ok(sb)
    }
}
