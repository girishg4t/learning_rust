use std::io;
use std::io::Write;

mod weight;

fn main() {
    print!("Enter 1 to run calculate weight on mars : ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.as_str().trim() {
        "1" => weight::do_calculate(),
        _ => {
            print!("not implemented")
        }
    }
}
