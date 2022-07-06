use serde_derive::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    address: Address,
    phones: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    city: String,
}

pub fn read_json() {
    let data = r#" {
        "name": "John Doe", "age": 43,
        "address": {"street": "main", "city":"Downtown"},
        "phones":["27726550023"]
       } "#;

    let p: Person = serde_json::from_str(data).expect("no json");
    println!("{}", p.address.city);
    println!("{:#?}", p);
}
