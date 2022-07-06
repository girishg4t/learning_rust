struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(f_name: String, l_name: String) -> Person {
        Person {
            first_name: f_name,
            last_name: l_name,
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

pub fn initialize_struct() {
    let p = Person::new("Girish".to_string(), "Talekar".to_string());
    println!("{}", p.full_name());
}
