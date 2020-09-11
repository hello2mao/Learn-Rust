trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>; // type Item和Self::Item ：定义trait的关联类型；
}
fn main() {
    let v1 = vec![1, 2, 3];
    for val in v1.iter() {
        println!("{}", val);
    }
    let mut v1_iter = v1.iter();
    if let Some(v) = v1_iter.next() {
        println!("{}", v);
    }
    if let Some(v) = v1_iter.next() {
        println!("{}", v);
    }
    if let Some(v) = v1_iter.next() {
        println!("{}", v);
    }
    if let Some(v) = v1_iter.next() {
        println!("{}", v);
    } else {
        println!("end")
    }

    //迭代可变引用
    let mut v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter_mut();
    if let Some(v) = v2_iter.next() {
        *v = 3;
    }
    println!("{:?}", v2);

    //消费适配器
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); //调用消费适配器sum来求和
    println!("total = {}", total);

    //迭代适配器
    let v1 = vec![1, 2, 3];
    let res: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", res);

    let v3: Vec<_> = v1.into_iter().filter(|x| *x > 2).collect();
    println!("{:?}", v3);


    println!("Hello, world!");
}
