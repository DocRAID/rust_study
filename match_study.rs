fn main() {
    let x = 50;
    match x {
        0 => {
            println!("이건 0임");
        }
        1 | 2 => {
            println!("1이나 2");
        }
        3..=12 => {
            println!("3에서 12 이하");
        }
        matched_num @ 22..=100 => {
            println!("{}",matched_num);
        }
        _ => {
            println!("아무것도 아님");
            
        }
    }
}
//오 이런게 있는줄은 알았는데 이렇게 쓰는거구나