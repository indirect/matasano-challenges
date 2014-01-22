pub trait Xor {
    fn xor(&self, other: &[u8]) -> ~[u8];
    fn xor_byte(&self, other: u8) -> ~[u8];
}

impl<'a> Xor for &'a [u8] {
    fn xor(&self, other: &[u8]) -> ~[u8] {    
        std::vec::build(Some(self.len()), |push| {
            for i in range(0, self.len()) {
                let a = self.get(i).unwrap();
                let b = other.get(i % other.len()).unwrap();
                push(a.bitxor(b));
            }
        })
    }

    fn xor_byte(&self, byte: u8) -> ~[u8] {
        let other = std::vec::build(Some(self.len()), |push| {
            self.len().times(|| push(byte));
        });
        self.xor(other)
    }
}
