#[crate_id = "p#0.1"];
#[crate_type = "lib"];

#[feature(macro_rules)];

#[macro_export];
macro_rules! p(
    ($ident:ident) => (
        println!("{:?}", $ident);
    );
)