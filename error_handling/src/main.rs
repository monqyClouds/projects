// Recovering from errors using "Result"
use std::fs::{File, read_to_string};
use std::io;
use std::io::{Read, ErrorKind};

fn main() {
    //// Method 1.

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    // Method 2.
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error)
    //     }
    // });

    //// Method 3.
    // let f = File::open("hello.txt").unwrap();

    //// Method 4.
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // let username = read_username_from_file_2();
}

// Propagating error method 1.
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Propagating error method 2; (using the "?" operator).
fn read_username_from_file_2 () -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Propagating error method 3;
fn read_username_from_file_3 () -> Result<String, io::Error> {
    read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}



//// Using panic! backtrace
// fn main() {
//     let v = vec![1,2,3];

//     v[99];
// }
