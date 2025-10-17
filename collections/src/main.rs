use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    let a = [1, 2, 3];
    let mut v:Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let third = &v[2];
    println!("The third element is {}", third);

    let index = 2;
    match v.get(index) {
        Some(element) => println!("The {} element is {}", index + 1, element),
        None => println!("There is no {} element", index + 1)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    for r in &row {
        match *r {
            SpreadsheetCell::Int(num) => println!("{}", num),
            _ => println!("Not an integer!")
        };
    }

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let score = scores.get(&String::from("Blue"));
    
}
