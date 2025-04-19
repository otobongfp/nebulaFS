use nebula_fs::fs::block::BlockDevice;
use nebula_fs::fs::superblock::Superblock;
use std::fs;

#[test]
fn test_superblock_read_write() {
    let path = "test_sb.bin";
    let _ = fs::remove_file(path);

    let mut device = BlockDevice::open(path).unwrap();
    let sb = Superblock::new(8192, 256);
    sb.write_to(&mut device).unwrap();

    let read_back = Superblock::read_from(&mut device).unwrap();
    assert_eq!(read_back.magic, Superblock::MAGIC);
    assert_eq!(read_back.total_blocks, 8192);
    assert_eq!(read_back.inode_count, 256);

    let _ = fs::remove_file(path);
}
