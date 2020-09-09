fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("abcd");
    let s2 = String::from("edf");
    let r = longest(s1.as_str(), s2.as_str());
    println!("r = {}", r);
    println!("Hello, world!");
}
