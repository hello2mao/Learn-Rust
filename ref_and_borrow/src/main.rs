// ref 引用：&
fn cal_length(s: &String) -> usize {
    s.len()
}

// borrow 借用：&mut
fn modify_s(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let mut s = String::from("hello");
    println!("s: {}", s);

    let len = cal_length(&s);
    println!("s len: {}", len);

    modify_s(&mut s);
    println!("new s: {}", s);

    println!("Hello, world!");
}
