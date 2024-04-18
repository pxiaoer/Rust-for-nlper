fn main() {
    
    //元组 (tuple),元组的元素，可以是不同的类型
    //元组没有元素变成unit类型，表示空元组
    let x:(i32,f64,u8) = (500,6.4,1);
    let a = x.0;
    let b = x.1;

    println!("tup is {:?}",x);


    //结构体 struct
    struct User {
        username:String,
        email:String,
        sign_in_count:u64,
        active:bool,
    }

    let user1 = User {
        username:String::from("pxiaoer"),
        email:String::from("pxiaoer@pxiaoer.blog"),
        sign_in_count:1,
        active:true,
        
    };

    println!("user1 is {:?}",user1.username);


    //枚举 enum
    #[derive(Debug)]
    enum Gender {
        GIRL,
        BOY,
    }

    let a = Gender::GIRL;
    println!("a is {:?}",a);
}