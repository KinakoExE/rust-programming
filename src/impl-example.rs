struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn say_name(&self) {
        println!("I am {}", self.name);
    }

    fn say_age(&self) {
        println!("I am {} yo", self.age);
    }
}

fn main() {
    let p = Person {
        name: String::from("kinako"),
        age: 24,
    };
    p.say_name();
    p.say_age();
}
