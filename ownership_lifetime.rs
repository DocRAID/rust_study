fn main(){
    let x;
    {
        let y= 20;
        x=y; // 소유권을 이전하기 때문에 오류를 달고 컴파일이 됨.
        // x=&y; 소유권을 대여 하는 형식이기 때문에 x가 수명이 y 보다 짧아서 참조 불가.
        // x = y(주소) = 20 이런형식
    }
    println!("{}",x);
    // and ...
    let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");

    let result = longest_word(&magic1, &magic2);
    println!("The longest magic word is {}", result);

    // let magic1 = String::from("abracadabra!");
    // let result;
    // {
    //     let magic2 = String::from("shazam!");
    //     result = longest_word(&magic1, &magic2);
    // }
    // println!("The longest magic word is {}", result);
}
// fn longest_word(x: &String, y: &String) -> &String
fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//https://docs.microsoft.com/ko-kr/learn/modules/rust-memory-management/3-validate-references-with-lifetimes