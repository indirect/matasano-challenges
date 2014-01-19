extern mod decrypt;

use std::path::Path;
use std::io::fs::File;
use std::io::buffered::BufferedReader;

#[test]
fn test_problem4_answer() {
    let path = Path::new(file!() + "/../gist.txt");
    let mut reader = BufferedReader::new(File::open(&path).unwrap());
    let mut answers = reader.lines().filter_map(|line| decrypt::xor_byte(line) );
    let best = answers.max_by(|answer| answer.score ).unwrap();

    assert_eq!(best.text, ~"Now that the party is jumping\n")
    assert_eq!(best.key, ~"5")
}
