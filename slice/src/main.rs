// 1.字符串slice是String中一部分值的引用
// 2.字面值就是slice
// 3.其他类型slice
fn main() {
    let s = String::from("hello world");

    let h = &s[0..5];
    println!("h: {}", h);

    let s1 = "hh"; // &str
    println!("s1: {}", &s1[..1]);

    let a = [1, 2, 3, 4];
    let array = &a[..1];
    println!("a: {}", array[0]);

    println!("Hello, world!");
}
