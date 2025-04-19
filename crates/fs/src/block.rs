use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

pub const BLOCK_SIZE: usize = 4096;

pub struct BlockDevice {
    file: File,
}

impl BlockDevice {
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        Ok(Self { file })
    }

    /// Read block
    pub fn read_block(&mut self, index: u64) -> std::io::Result<[u8; BLOCK_SIZE]> {
        let mut buffer = [0u8; BLOCK_SIZE];

        // Seek to the correct position: block_index * BLOCK_SIZE
        self.file.seek(SeekFrom::Start(index * BLOCK_SIZE as u64))?;
        self.file.read_exact(&mut buffer)?;

        Ok(buffer)
    }

    /// Write to block
    pub fn write_block(&mut self, index: u64, data: &[u8; BLOCK_SIZE]) -> std::io::Result<()> {
        self.file.seek(SeekFrom::Start(index * BLOCK_SIZE as u64))?;
        self.file.write_all(data)?;
        Ok(())
    }
}
