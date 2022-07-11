use std::fs::File;

pub fn do_unwrap() {
    let f1 = File::open("hello.txt").expect("Failed to open hello.txt");
    println!("{:?}   ", f1);

    let f = File::open("hello.txt").unwrap();
    println!("{:?}", f);
}
