use crates::utils;
use crates::{PrimaryColor, SecondaryColor};

fn main() {
    let red = PrimaryColor::Red;
    let blue = PrimaryColor::Blue;

    match utils::mix(red, blue) {
        Ok(value) => match value {
            SecondaryColor::Purple => println!("result is purple"),
            SecondaryColor::Green => println!("result is green"),
            SecondaryColor::Orange => println!("result is orange"),
        },
        Err(e) => println!("{e}"),
    };
}
