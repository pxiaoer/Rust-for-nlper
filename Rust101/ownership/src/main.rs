
fn foo(s: String) -> String {
    println!("{}", s);
    s
}


fn foo1(s: &String) {
    println!("{}", s);
}


fn foo2(s: &mut String) {
    s.push_str(" world");
}


fn main() {

    let s1 = String::from("hello, I am pxiaoer");
    let s2 = s1;

    //println!("s1 is {}",s1); //error: value borrowed here after move  s1复制给s2后，s1的所有权已经转移给s2，所以s1不能再使用
    println!("s2 is {}",s2);

    let s3 = String::from("hello, I am pxiaoer");
    let s4 = s3.clone();//deep copy s3 clone给s4后，s3和s4都有自己的内存空间，互不影响

    println!("s3 is {}",s3);
    println!("s4 is {}",s4);

    foo(s4);//s4的所有权转移给foo函数，所以s4不能再使用
    //println!("s4 is {}",s4);//error: value borrowed here after move


    let s5 = String::from("hello, I am pxiaoer");
    foo(s5);//s5的所有权转移给foo函数，所以s5不能再使用
    //foo(s5);//error: value borrowed here after move  重复调用也是不行的

    let s6 = String::from("hello, I am pxiaoer");
    let s6 = foo(s6);
    println!("s6 is {}",s6);//s6的所有权转移给foo函数，foo函数返回s6，所以s6可以继续使用


    //引用

    let mut a = 42u32;
    let b = &mut a;
    *b += 1;
    println!("{}", a);//43

    let c = &a;
    println!("{}", c);//43

    //println!("{}", b); //mutable borrow later used here b是可变引用，c是不可变引用，不可变引用和可变引用不能同时存在
    //引用型变量的作用域是从它定义起到它最后一次使用时结束


    let s7 = String::from("hello, I am pxiaoer");
    foo1(&s7);//s7的所有权没有转移给foo1函数，所以s7可以继续使用
    println!("s7 is {}",s7);


    let mut s8 = String::from("hello, I am pxiaoer");
    foo2(&mut s8);//s8的所有权没有转移给foo2函数，所以s8可以继续使用,而且还可以修改
    println!("s8 is {}",s8);

}


