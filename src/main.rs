use std::io;
use std::io::Write;

mod external_json;
mod looping;
mod reverse;
mod structs;
mod traits;
mod tree;
mod tuple;
mod weight;

fn main() {
    print!("Enter 1 to run calculate weight on mars : ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.as_str().trim() {
        "1" => weight::do_calculate(),
        "2" => reverse::reverse_string(),
        "3" => looping::doloop(),
        "4" => looping::call_ref(),
        "5" => looping::total(),
        "6" => looping::compare_string(),
        "7" => tuple::do_tuple(),
        "8" => structs::initialize_struct(),
        "9" => traits::show_something(),
        "10" => external_json::read_json(),
        _ => {
            print!("not implemented")
        }
    }
}
