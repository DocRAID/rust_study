fn fn1(value:&str) -> String{
    println!("{}", value);

    ("return!").to_string()
}
fn main(){
    let hello : &str = "i'm funcion!";
    println!("{}", fn1(hello));
}
