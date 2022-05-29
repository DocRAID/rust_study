fn print_exist(message:&String){
    println!("{}",message);
}
// fn change(text:&String){
//     text.push_str(" and changed");
// }
fn change(text:&mut String){
    text.push_str(" and changed");
}
fn main(){
    let mut rent=String::from("the value is exist");
    let rent_reference = &rent;
    println!("holy moly... value is exist??? \nans: {}",rent);

    print_exist(&rent);
    print_exist(&rent);// 대여하기 때문에 더 참조 쌉가능

    // change(&rent);
    change(&mut rent);
    //소유권이 change() 에 갔다 오기때문에 값이 변해서 옴
    println!("{}",rent);
}