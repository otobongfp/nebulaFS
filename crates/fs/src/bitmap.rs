use std::io::{Read, Seek, SeekFrom, Write};

use crate::block::{BlockDevice, BLOCK_SIZE};

pub struct Bitmap {
    pub start_block: u64,     // Where bitmap begins on disk
    pub num_blocks: u64,      // Number of data blocks being tracked
    pub bits: Vec<u8>,        // Actual bitmap (1 bit per block)
}

impl Bitmap {
    pub fn new(start_block: u64, num_blocks: u64) -> Self {
        let size_in_bytes = ((num_blocks + 7) / 8) as usize;
        Self {
            start_block,
            num_blocks,
            bits: vec![0u8; size_in_bytes],
        }
    }

    pub fn allocate(&mut self) -> Option<u64> {
        for (byte_index, byte) in self.bits.iter_mut().enumerate() {
            if *byte != 0xFF {
                for bit in 0..8 {
                    let mask = 1 << bit;
                    if *byte & mask == 0 {
                        *byte |= mask;
                        let block_id = (byte_index * 8 + bit) as u64;
                        if block_id < self.num_blocks {
                            return Some(block_id);
                        } else {
                            return None; // out of bounds
                        }
                    }
                }
            }
        }
        None // No free block
    }

    pub fn deallocate(&mut self, block_id: u64) {
        if block_id >= self.num_blocks {
            return;
        }
        let byte_index = (block_id / 8) as usize;
        let bit_index = (block_id % 8) as u8;
        self.bits[byte_index] &= !(1 << bit_index);
    }

    pub fn is_allocated(&self, block_id: u64) -> bool {
        if block_id >= self.num_blocks {
            return false;
        }
        let byte_index = (block_id / 8) as usize;
        let bit_index = (block_id % 8) as u8;
        (self.bits[byte_index] & (1 << bit_index)) != 0
    }

    pub fn save(&self, device: &mut dyn BlockDevice) -> std::io::Result<()> {
        let mut remaining = self.bits.as_slice();
        let mut block_id = self.start_block;

        while !remaining.is_empty() {
            let mut buf = [0u8; BLOCK_SIZE];
            let to_copy = remaining.len().min(BLOCK_SIZE);
            buf[..to_copy].copy_from_slice(&remaining[..to_copy]);
            device.write_block(block_id, &buf)?;
            block_id += 1;
            remaining = &remaining[to_copy..];
        }

        Ok(())
    }

    pub fn load(device: &mut dyn BlockDevice, start_block: u64, num_blocks: u64) -> std::io::Result<Self> {
        let size_in_bytes = ((num_blocks + 7) / 8) as usize;
        let mut bits = vec![0u8; size_in_bytes];
        let mut remaining = bits.as_mut_slice();
        let mut block_id = start_block;

        while !remaining.is_empty() {
            let mut buf = [0u8; BLOCK_SIZE];
            device.read_block(block_id, &mut buf)?;
            let to_copy = remaining.len().min(BLOCK_SIZE);
            remaining[..to_copy].copy_from_slice(&buf[..to_copy]);
            remaining = &mut remaining[to_copy..];
            block_id += 1;
        }

        Ok(Self {
            start_block,
            num_blocks,
            bits,
        })
    }
}
