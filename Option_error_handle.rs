fn one(){
    let v = vec![0, 1, 2, 3];
    println!("{}", v[6]);
    panic!("Fire!!!");
}
fn two(){
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    
    let first = fruits.get(0);
    println!("{:?}", first);
    let third = fruits.get(2);
    println!("{:?}", third);
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);
}
fn three(){
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0,2,99].iter(){
        match fruits.get(index){
            Some(&"coconut") => println!("Coconut is best!"),
            Some(fruit_name) => println!("It's a delicious {}",fruit_name),
            None => println!("There is no fruit! :("),
        }
    }
}
fn four(){
    let a_number: Option<u8> = Some(7);
    if let Some(7) = a_number {
        println!("That's seven!!");
    }
}
fn five(){
    let gift = Some("candy");
    assert_eq!(gift.unwrap(),"candy");
    
    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "cat");
}
fn main(){
    // one();
    // two();
    // three();
    // four();
    five();
}