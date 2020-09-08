//1、创建空的vector：Vec<T>
//2、创建包含初始值的vec
//3、丢弃vec
//4、读取元素
//5、更新元素
//6、遍历
//7、使用枚举
fn main() {

    //1
    let mut v: Vec<i32> = Vec::new();
    //v.push(1);

    //2
    let mut v = vec![1, 3, 4];

    //3
    {
        let v1 = vec![1, 2, 3];
    }

    //4
    let one = v[0];
    println!("one = {}", one);

    //recommend
    match v.get(10) {
        Some(value) => println!("value = {}", value),
        None => println!("get nothing")
    }

    //5
    v.push(11);
    v.push(22);
    v.push(33);

    //6
    //(1)不可变的遍历
    for i in &v {
        println!("i = {}", i);
    }
    //(2)可变的遍历
    for i in &mut v {
        *i += 1;
        println!("i = {}", i);
    }

    //7
    #[derive(Debug)]
    enum Context {
        Text(String),
        Float(f32),
        Int(i32),
    };
    let c = vec![
        Context::Text(String::from("haha")),
        Context::Int(-1),
    ];
    for i in &c {
        println!("i = {:#?}", i);
    }



    println!("Hello, world!");
}
