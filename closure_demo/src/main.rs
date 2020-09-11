//闭包是可以保存进变量或者作为参数传递给其他函数的匿名函数。闭包和函数不同，闭包允许捕获调用者作用域中的值。
fn main() {
    let use_closure = || {
        println!("this is a closure");
    };
    use_closure();

    let add_one_v2 = |x: u32| -> u32 { x + 1};
    let add_one_v3 = |x| { x + 1};
    let add_one_v4 = |x| x+1;
    println!("{} {} {}", add_one_v2(2), add_one_v3(2), add_one_v4(2));

    //不能推导两次；

    //捕捉环境中的变量
    let y = 1;
    let exe = |x| x + y;
    println!("{}", exe(2));

    //move
    let eauql_to_x = move |x| x==y;
    println!("{}", eauql_to_x(1));

    println!("Hello, world !");
}

fn add_one_v1(x: u32) -> u32 {
    x + 1
}


