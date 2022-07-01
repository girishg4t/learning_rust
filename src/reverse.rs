use std::io;
use std::io::Write;

pub fn reverse_string() {
    print!("Enter string to be reversed : ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut res: String = String::new();
    for i in input.chars() {
        res = i.to_string() + &res;
    }
    print!("{}\n", res)
}
