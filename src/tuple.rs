fn add_mult(x: i32, y: i32) -> (i32, i32) {
    (x * y, x + y)
}

pub fn do_tuple() {
    let res = add_mult(10, 20);

    println!("{:?}", res);
    println!("{},{}", res.0, res.1);

    zip_tuple();
}

fn zip_tuple() {
    let nums = [10, 100, 200];
    let str_num = ["one", "two", "three"];

    for t in str_num.iter().zip(nums.iter()) {
        println!("{}, {}", t.0, t.1);
    }
}
