use nebula_fs::fs::block::{BlockDevice, BLOCK_SIZE};
use std::fs;

#[test]
fn test_block_read_write() {
    let path = "test_block.bin";
    let _ = fs::remove_file(path);

    let mut device = BlockDevice::open(path).unwrap();

    let block = [42u8; BLOCK_SIZE];
    device.write_block(0, &block).unwrap();

    let read_back = device.read_block(0).unwrap();
    assert_eq!(read_back, block);

    let _ = fs::remove_file(path);
}
