struct Dog {
    name: String,
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("dog leave with name = {}", self.name);
    }
}

fn main() {
    let a = Dog {name: String::from("Tom")};
    {
        let b = Dog {name: String::from("Alice")};
    }

    println!("Hello, world!");
}
