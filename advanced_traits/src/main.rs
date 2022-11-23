use std::ops::Add;

mod assosciated_types;
mod newtype_pattern;
mod supertraits;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

use std::fmt;

// implemeting fmt::display as a supertrait of Point
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl supertraits::OutlinePrint for Point {}

// calling methods with same name
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiosly*")
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

#[allow(dead_code)]
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    // Calling methods with the same name on a type
    let person = Human;
    Wizard::fly(&person);
    Pilot::fly(&person);
    Human::fly(&person); // same as person.fly()

    // <Human as Wizard>::fly(&person);    // same as person.fly()

    // using fully qualified syntax
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // Fully Qualified Syntax is defined as follows
    // <Type as Trait>::function(reciever_if_method, next_arg, ...);

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // newtype pattern
    newtype_pattern::wrapper_print();
}
