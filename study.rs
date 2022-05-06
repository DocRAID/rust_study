fn main(){
    println!("Hello!");
    // todo!("ㅋㅋㅋ 아 이거 고쳐야함");
    println!("{1} 이건 이렇게 {0} 이건 이렇게",1,2);

    let mut hello_int:i64 = -10;
    // let hello_unsigned:u64 = 10;
    println!("{hello_int}");

    //mut 가 변수가 바뀌게 해주는것.
    hello_int = 11;
    println!("{hello_int}"); 

    let is_bigger = 1>4;
    println!("{is_bigger}");

    let string:&str = "hello";
    println!("{string}");

    let tuple = ('a', 20i32, false, 2);
    println!("{} and {} and {} and {}",tuple.0,tuple.1,tuple.2,tuple.3);

    struct Student {
        name:String,
        age:i64,
        attend:bool
    }
    //String::from("Limdongju")
    let lim = Student{name: ("Limdongju").to_string(),age:18,attend:true};
    println!("{} {} {}",lim.name,lim.age,lim.attend);
}