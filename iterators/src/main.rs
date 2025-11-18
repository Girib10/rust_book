fn main() {
    let v1 =vec![1, 2, 3];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("Got {}", value);
    }

    let v2: Vec<i32> = v1.iter().map(|x| x + 3).collect();

     for value in v2.iter() {
        println!("Got {}", value);
    }
}
