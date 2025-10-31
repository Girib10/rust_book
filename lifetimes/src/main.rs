fn main() {
    let string1 = String::from("abcd");

    // This works
    {
        let string2= String::from("xyz");

        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // This doesn't
    // let result;
    // {
    //     let string2= String::from("xyz");

    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
}

// The lifetime of the return value is equal to the smallest lifetime
// between x and y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
