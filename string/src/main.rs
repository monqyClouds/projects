fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    let mut s = String::from("foo");
    let s2 = " bar";
    s.push_str(s2);

    let mut s3 = String::from("lo");
    s3.push('l');       // Single char concatenation

    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let s6 = s4 + &s5; // takes ownership of the first variable

    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");

    let s10 = format!("{}-{}-{}", s7, s8, s9); // doesnt take ownership

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
