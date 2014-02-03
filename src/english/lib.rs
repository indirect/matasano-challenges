#[crate_id = "english#0.1"];
#[crate_type = "lib"];

pub fn score(bytes: &[u8]) -> int {
    bytes.iter().fold(0, |s, &x| {
        match x {
            // letters are just a frequency table from the internet
            69 | 101 => s + 120,
            84 | 116 => s + 90,
            65 | 97 => s + 80,
            73 | 105 => s + 80,
            78 | 110 => s + 80,
            79 | 111 => s + 80,
            83 | 115 => s + 80,
            72 | 104 => s + 64,
            82 | 114 => s + 62,
            68 | 100 => s + 44,
            76 | 108 => s + 40,
            85 | 117 => s + 34,
            67 | 99 => s + 30,
            77 | 109 => s + 30,
            70 | 102 => s + 25,
            87 | 119 => s + 20,
            89 | 121 => s + 20,
            71 | 103 => s + 17,
            80 | 112 => s + 17,
            66 | 98 => s + 16,
            86 | 118 => s + 12,
            75 | 107 => s + 8,
            81 | 113 => s + 5,
            74 | 106 => s + 4,
            88 | 120 => s + 4,
            90 | 122 => s + 2,
            // first we count \t\n\r as tiny but okay
            9 | 10 | 13 => s + 1,
            // spaces are pretty likely
            32 => s + 50,
            // other ASCII characters are okay I guess
            33..126 => s + 1,
            // then we count any control chars as death
            0..31 => s - 10000,
            // anything we don't know about yet is probably bad
            _ => s - 10000
        }
    })
}
