fn main() {
   

    //rust是区分位数的，有长度和有无符号位组成:isize,usize
    let a: u32 = 1;
    let b = 10i128;
    let c = 42.0f32;
    println!("{},{},{}",a,b,c);


    //bool类型有true和false，字符类型单引号引用，字符是unicode值
    let a = true;
    let b: bool = false;
    let c: char = 'Z';
    println!("{},{},{}",a,b,c);


    //字符串类型为String，字符串并不是字符数组要注意，也不能通过下标访问
    let hello=String::from("你好,");
    let world = String::from("word!");

    let helloworld = String::from("你好\n,世界");

    println!("{},{},{}",hello,world,helloworld);


    //字节串，b开头
    let hello: &[u8;21] =  b"this is a byte string";
    println!("{:?}", hello);



    //数组,arry类型，存储同一类型的多个值，可以下标访问

    let a: [i32;5]= [1,2,3,4,5];
    let b =[1,2,3];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];


    println!("{},{},{}",a[0],b[1],months[11]);



    //动态数组Vec,mut代表可变
    let a: Vec<i32> = Vec::new();
    let a = vec![1,2,3];

    let mut b = Vec::new();
    b.push(1);
    b.push(2);
    b.push(3);

    println!("{},{}",a[0],b[1]);


    //哈希表HashMap

    use std:: collections::HashMap;

    let mut months = HashMap::new();
    months.insert(1,String::from("January"));
    months.insert(2,String::from("Feburary"));

    println!("{:?}",months);
    
}