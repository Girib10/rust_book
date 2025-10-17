use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    let f = File::open("hello.txt").expect("Failed to open file hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error createing file: {:?}", e)
    //         }
    //         other_error=> {
    //             panic!("Error opening file: {:?}", other_error)
    //         }
    //     }
    // };

    read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;

    // 1
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e)
    // };

    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)

    // 1
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e)
    // }
}
