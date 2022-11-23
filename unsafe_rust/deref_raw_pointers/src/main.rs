fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        // *r2 += 3;

        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
