// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.


use wwj::my_macro;

fn main() {
    my_macro!();
}

mod wwj {
    macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
        };
    }
    pub(crate) use my_macro;
}