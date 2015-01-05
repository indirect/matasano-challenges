pub fn pkcs7(block_size: uint, bytes: &[u8]) -> Vec<u8> {
    let padding_size = block_size - (bytes.len() % block_size);
    let pad = padding_size as u8;

    let mut padded_bytes = bytes.to_vec();
    for _ in range(0, padding_size) {
        padded_bytes.push(pad);
    }

    padded_bytes
}

pub fn count_repeated_blocks(bytes: &[u8], block_size: uint) -> uint {
    let block_count = bytes.len() / block_size as uint;

    let mut repeated_blocks: uint = 0;
    for block_idx in range(0, block_count) {
        let block = bytes.slice(block_idx * block_size, (block_idx + 1) * block_size);
        for other_idx in range(block_idx + 1, block_count) {
            if block == bytes.slice(other_idx*block_size, (other_idx+1)*block_size) {
                repeated_blocks += 1;
            }
        }
    }

    repeated_blocks
}

#[test]
fn no_repeated_blocks() {
    let bytes = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(0, count_repeated_blocks(bytes.as_slice(), 3));
}

#[test]
fn one_repeated_block() {
    let bytes = vec![1, 1, 2, 3, 4, 5, 1, 1, 6, 7];
    assert_eq!(1, count_repeated_blocks(bytes.as_slice(), 2));
}

#[test]
fn three_repeated_blocks() {
    let bytes = vec![1, 1, 2, 3, 1, 1, 4, 5, 1, 1, 6, 7];
    assert_eq!(3, count_repeated_blocks(bytes.as_slice(), 2));
}
