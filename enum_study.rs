#[derive(Debug)]
enum Enum {
    A(String),
    B(String)
}

fn main(){
    let a = Enum::A("ITISA".into());
    let b = Enum::B("ITISB".into());
    println!("{:?}",a);

}