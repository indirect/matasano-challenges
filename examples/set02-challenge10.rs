extern crate crypto;
extern crate matasano_challenges;

fn main() {
    let data = matasano_challenges::bytes_from_data_file("set02-challenge10.txt");
    let key = "YELLOW SUBMARINE".as_bytes();
    let result = crypto::cbc::decrypt_zero_iv(
        key.as_slice(),
        data.as_slice()
    );
    let plaintext = String::from_utf8_lossy(result.as_slice());

    println!("{:?}", plaintext);
}
