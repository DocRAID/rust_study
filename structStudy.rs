struct Phone {
    color: String,
    manufacturer: Manufacturer,
    security: bool,
    size: i32,
}
 #[derive(Debug)]
 // 리턴 제공
enum Manufacturer{
    Apple,
    Samsung,
    Lg,
}
// enum Manufacturer{
//     Apple,
//     Samsung,
//     Lg,
// }

// impl PartialEq for Manufacturer {
//     fn eq(&self, other: &Manufacturer) -> bool {
//         match (*self, *other) {
//             (Apple, Apple) => true,
//             (Samsung, Samsung) => true,
//             (Lg, Lg) => true,
//             (_, _) => false,
//         }
//     }
// }

// impl std::fmt::Debug for Manufacturer {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//         formatter.write_str(match *self {
//             Apple => "Apple",
//             Samsung => "Samsung",
//             Lg => "Lg",
//         })
//     }
// }
fn phone_factory(color: String, manufacturer: Manufacturer, security:bool) -> Phone{
    Phone{
        color: color,
        manufacturer: manufacturer,
        security: security,
        size: 10
    }
}
fn main(){
    let mut phone = phone_factory(String::from("Black"), Manufacturer::Apple, true);
    println!("phone1: {}, {:?} factory, security: {}, size:{}",phone.color,phone.manufacturer,phone.security,phone.size);
    
    phone = phone_factory(String::from("Pink"), Manufacturer::Samsung, true);
    println!("phone2: {}, {:?} factory, security: {}, size:{}",phone.color,phone.manufacturer,phone.security,phone.size);
    
    phone = phone_factory(String::from("Yellow"), Manufacturer::Lg, false);
    println!("phone3: {}, {:?} factory, security: {}, size:{}",phone.color,phone.manufacturer,phone.security,phone.size);

    let v= [1,2,3];
    println!("{:?}",v);
}