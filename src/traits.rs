trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("show me {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("show me {}", self)
    }
}

pub fn show_something() {
    let i = 10;
    let f = 1.2;

    println!("{}", i.show());
    println!("{}", f.show());
}
