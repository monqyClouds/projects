// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

mod front_of_house;

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub use crate::front_of_house::hosting;
use back_of_house::{Appetizer, Breakfast};

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;

    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("wheat");

    println!("I'd like {} toast please", meal.toast);
}
