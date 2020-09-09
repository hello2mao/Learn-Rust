pub trait GetInfo {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
    fn get_school_name(&self) -> String {
        String::from("123")
    }
}

pub struct Student {
    pub name: String,
    pub age: u32,
}

impl GetInfo for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

fn print_name_1(item: impl GetInfo) {
    println!("name: {}", item.get_name());
}

fn print_name_2<T: GetInfo>(item: T) {
    println!("name: {}", item.get_name());
}

fn main() {
    let xiaoming = Student {
        name: String::from("xiaoming"),
        age: 12,
    };
    println!("name = {}", xiaoming.get_name());
    println!("school name = {}", xiaoming.get_school_name());
    print_name_1(xiaoming);
    print_name_2(xiaoming);

    println!("Hello, world!");
}
