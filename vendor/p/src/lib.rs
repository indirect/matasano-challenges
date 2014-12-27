#![feature(macro_rules)]

macro_rules! p(
    ($ident:ident) => (
        println!("{:?}", $ident);
    );
);

#[test]
fn it_accepts_literals() {
  let foo = 12u;
  p!(foo);
}
