use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

// Simulating memory leak with reference cycle
#[derive(Debug)]
enum List2 {
    Cons2(i32, RefCell<Rc<List2>>),
    Nil2,
}

impl List2 {
    fn tail(&self) -> Option<&RefCell<Rc<List2>>> {
        match self {
            Cons2(_, item) => Some(item),
            Nil2 => None,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

use crate::List::{Cons, Nil};
use crate::List2::{Cons2, Nil2};

#[allow(dead_code)]
fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let _b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let _c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    // println!("a after = {a:?}");
    // println!("b after = {b:?}");
    // println!("c after = {c:?}");

    let a = Rc::new(Cons2(5, RefCell::new(Rc::new(Nil2))));

    // println!("a initial rc count = {}", Rc::strong_count(&a));
    // println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons2(10, RefCell::new(Rc::clone(&a))));

    // println!("a rc count after b ceation is = {}", Rc::strong_count(&a));
    // println!("b initial rc count = {}", Rc::strong_count(&b));
    // println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Causes stack overflow
    // println!("a next item = {:?}", a.tail());

    // Using Weak<T> for reference counting instead of strong count.
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
