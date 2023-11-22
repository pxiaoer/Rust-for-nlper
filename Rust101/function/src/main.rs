
//函数 fn
fn add(x:i32,y:i32) -> i32 {
    x + y
}


fn main() {
    let a = add(1,2);
    println!("a is {}",a);


    //闭包Closure
    //闭包是一个匿名函数，可以捕获上下文中的变量

    let add_a = |x| -> i32{x + a};

    let b = 10;
    let result = add_a(b);

    println!("result is {}",result);

}
