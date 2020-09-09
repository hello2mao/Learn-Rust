trait GetName {
    fn get_name(&self) -> &String;
}

trait GetAge {
    fn get_age(&self) -> u32;
}

//1
fn print_info_1<T: GetName + GetAge>(item: T) {
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

//2
fn print_info_2<T>(item: T)
where
    T: GetName + GetAge,
{
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

fn main() {
    println!("Hello, world!");
}
