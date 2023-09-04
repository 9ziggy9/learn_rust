trait Talks {
    fn speak(&self);
}

struct Cat {
    name:   String,
    weight: i32,
    age:    i32,
}

impl Talks for Cat {
    fn speak(&self) {
        println!("{} goes meow!", self.name);
    }
}

fn main() {
    let echo = Cat {
        name: String::from("Echo"),
        weight: 2,
        age: 1,
    };
    let charlie = {
        let charlie_name: String = String::from("Charlie");
        Cat {
            name: charlie_name,
            weight: 9,
            age: 3,
        }
    };
    echo.speak();
    charlie.speak();
}
