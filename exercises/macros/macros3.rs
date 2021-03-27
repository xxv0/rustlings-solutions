// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)


mod macros {
    #[macro_export]  //put macro in crate
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    pub fn test() {
        println!("Hello Macros");
    }
}

// use crate::my_macro;

fn main() {
    my_macro!();
    macros::test();
}
