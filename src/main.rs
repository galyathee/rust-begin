// This is single line comment - ignored by the compiler
/*
   This is
     a multi-line
       comment
 also ignored by the compiler
 */

mod primitives;
mod threads;
mod visibility_modifiers;

// the main function
fn main() {
    println!("Hello, world!");

    primitives::main();
    threads::main();
    visibility_modifiers::main();
}
