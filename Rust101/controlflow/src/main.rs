fn main() {

    //if else
    if 1 == 1 {
        println!("1 == 1");
    } else {
        println!("1 != 1");
    }


    //if else支持返回值
    let x =1;

    let y = if x == 1 {
        1
    } else {
        2
    };

    println!("y is {}",y);



    //循环语句：loop,while,for

    let mut x = 1;
    let mut sum = 0;
    loop {
        sum += x;
        x += 1;
        if x == 10 {
            break;
        }
    }
    println!("sum is {}",sum);


    let mut sum = 0;
    let mut x = 1;
    while x < 10 {
        sum += x;
        x += 1;
    }
    println!("sum is {}",sum);


    let mut sum = 0;
    for i in 1..10 {
        sum += i;
    }

    println!("sum is {}",sum);

}
