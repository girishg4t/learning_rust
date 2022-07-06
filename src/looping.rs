pub fn doloop() {
    let mut sum = 0;
    for i in 0..10 {
        sum = sum + i;
    }
    print!("{}", sum)
}

fn pass_by_ref(x: &i32) -> i32 {
    *x + 1
}

pub fn call_ref() {
    let i = 10;

    let res = pass_by_ref(&i);

    print!("{}", res)
}

fn sum_array(a: &[i32]) -> i32 {
    let mut res = 0;

    for i in a {
        res = res + i
    }
    res
}

pub fn total() {
    let arr = [10, 20, 30];
    let res = sum_array(&arr);
    print!("{}", res)
}

pub fn compare_string() {
    let mut s = String::new();
    // initially empty!
    s.push('H');
    s.push_str("ello");
    s.push(' ');
    s += "World!"; // short for `push_str`
                   // remove the last char
    s.pop();

    println!("{}", s);
}
