struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn say_name(&self) -> &Self {
        println!("I am {}", self.name);
        self
    }

    fn say_age(&self) -> &Self {
        println!("I am {} yo", self.age);
        self
    }
}

fn main() {
    let p = Person {
        name: String::from("kinako"),
        age: 24,
    };
    p.say_name().say_age();
}
