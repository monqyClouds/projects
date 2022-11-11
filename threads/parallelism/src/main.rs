use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // guranteeing that a thread is run to completion
    handle.join().unwrap();

    // using external variables inside thread closure
    let v = vec![1, 2, 3];

    // move keyword forces the closure the own its referenced variables
    let handle = thread::spawn(move || println!("Here's a vector: {v:?}"));

    handle.join().unwrap();
}
