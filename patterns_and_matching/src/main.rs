fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background");
    }

    // while let loop
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(value) = stack.pop() {
        println!("{value}")
    }

    // let statements as "let PATTERN = EXPRESSION;"
    let (_x, _y, _z) = (1, 2, 3);

    // matching named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // will match the below code because of shadowing of y variable
        Some(y) => println!("Matched, y = {y:?}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y:?}");

    // matching multiple patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"), // matches
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching ranges of values
    let x = 5;

    match x {
        1..=5 => println!("one through five"), // matches
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"), // matches
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // destructuring structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(a, x);
    assert_eq!(7, b);
    assert_eq!(b, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }

    // ignoring values with _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    let s = Some(String::from("Hello"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");

    // Ignoring remaining parts of a value with ..

    #[allow(dead_code)]
    struct Point2 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point2 { x: 0, y: 0, z: 0 };

    match origin {
        Point2 { x, .. } => println!("x is {x}"),
    }

    // extra conditonals with match guards
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n:?}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y:?}");

    // using the match guard with "or"/ "|" operator
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"), // doesnt match because y = false
        _ => println!("no"),
    }

    // @ bindings
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found on the id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}
