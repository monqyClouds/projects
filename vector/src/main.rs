fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);

    let v2 = vec![2,3,1,3,1];

    let _second = &v2[1];

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &mut v {
        *i += 50;
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.2),
        SpreadsheetCell::Text(String::from("Nice")),
    ];


}
