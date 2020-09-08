//enum Option<T> {
//  Some(T),
//  None,    
//}
fn main() {

    let some_number = Some(5);
    let some_string = Some(String::from("a string"));

    let x: i32 = 5;
    let y: Option<i32> = Some(5);
    let mut temp = 0;
    match y {
        Some(i) => {
            temp = i;
        },
        None => {
            println!("do nothing");
        }
    }
    let sum = x + temp;
    println!("sum is {}", sum);

    if let Some(value) = plus_one(y) {
        println!("value = {}", value);
    }

    println!("Hello, world!");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x+1),
        None => None,
    }
}
