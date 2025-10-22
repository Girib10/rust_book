struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T> Point<T, f64> {
    fn y(&self) -> &f64 {
        &self.y
    }
    
}

fn main() {
    let p1 = Point {x:5, y: 7};
    let p2 = Point {x:5.0, y: 7.0};
    let p3 = Point {x:5, y: 7.0};

    p3.y();

    let number_list = vec![1, 5, 7, 8, 13];
    println!("Largest in {:?} is {}", number_list, get_largest(&number_list))
}

fn get_largest<T: PartialOrd + Copy>(list: &Vec<T>) -> T {
    let mut largest = list[0];
    for value in list {
        if *value > largest {
            largest = *value;
        }
    }
    largest
}