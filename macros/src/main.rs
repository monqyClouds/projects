use hello_macro;

struct Pancakes;

use hello_macro::HelloMacro;

impl hello_macro::HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!")
    }
}

fn main() {
    println!("Hello, world!");
    Pancakes::hello_macro();
}
