fn main() {
    let a:u8 = 13;
    let b:i32 = 20;
    let c:i32 = a as i32 + b; //형변환 이렇게 함
    println!("{}",c);
}