#[derive(Debug)]
struct Dog {
    name: String,
    weight: f32,
}

impl Dog {
    fn get_name(&self) -> &str {
        &(self.name[..])
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn show(&self) {
        
    }
}

fn main() {
    let dog = Dog {
        name: String::from("wangcai"),
        weight: 100.0,
    };
    println!("dog: {:#?}", dog);
    println!("dog name: {}", dog.get_name());
    println!("dog weight: {}", dog.get_weight());


    println!("Hello, world!");
}
