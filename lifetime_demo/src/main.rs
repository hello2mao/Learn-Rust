fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct StuA<'a> {
    name: &'a str,
}

impl<'a> StuA<'a> {
    fn do_something(&self) -> i32 {
        3
    }

    fn do_something_2(&self, s: &str) -> &str {
        self.name
    }

    fn do_something_3<'b>(&self, s: &'b str) -> &'b str {
        s
    }
}

fn main() {
    let s1 = String::from("abcd");
    let s2 = String::from("edf");
    let r = longest(s1.as_str(), s2.as_str());
    println!("r = {}", r);
    println!("Hello, world!");
}
