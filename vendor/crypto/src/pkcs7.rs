pub fn pad(bytes: &[u8], block_size: uint) -> Vec<u8> {
    let padding_size = block_size - (bytes.len() % block_size);
    let pad = padding_size as u8;

    let mut padded_bytes = bytes.to_vec();
    for _ in range(0, padding_size) {
        padded_bytes.push(pad);
    }

    padded_bytes
}
