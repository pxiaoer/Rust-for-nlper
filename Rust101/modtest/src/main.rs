mod add;


//配置测试模块 cargo test启动测试
#[cfg(test)]
mod tests {
    use crate::add::tupe_add::tupe_add;

    #[test]
    fn test_tupe_add() {
        let a = (1, 2);
        let b = (3, 4);
        let c = tupe_add(a, b);
        assert_eq!(c, (4, 6));
    }
}


fn main() {
    let c =add::tupe_add::tupe_add((1,2),(3,4));
    println!("c is {:?}",c);

}
