fn main() {
    //1、定义结构体
    #[derive(Debug)]
    struct User {
        name: String,
        count: String,
        nonce: u64,
        active: bool,
    }

    //2、创建结构体实例
    let mut xiaoming = User {
        name: String::from("xiaoming"),
        count: String::from("1000"),
        nonce: 123,
        active: true,
    };

    //3、修改结构体字段
    xiaoming.nonce = 456;

    //4、参数名字和字段名字同名的简写方法
    let count = String::from("2212");
    let nonce = 334;
    let active = false;
    let xiaohuang = User {
        name: String::from("xiaohuang"),
        count,
        nonce,
        active,
    };

    //5、从其他结构体创建实例
    let xiaohua = User {
        name: String::from("xiaohua"),
        ..xiaohuang
    };
    println!("xiaohua: {:?}", xiaohua);

    //6、元组结构体
    #[derive(Debug)]
    struct Point(i32, i32);
    let a = Point(10, 20);
    println!("a: {:?}", a);

    //7、没有任何字段的类单元结构体
    struct A{};

    //8、打印结构体
    println!("xiaoming: {:?}", xiaoming);

    println!("Hello, world!");
}
