use std::sync::Mutex;
struct Pie;
impl Pie {
    fn eat(&self){
        for i in (1..=100){
            println!("{}",i);
        }
    }
}
fn main() {
    let mutex_pie = Mutex::new(Pie);
    let ref_pie = mutex_pie.lock().unwrap();
    ref_pie.eat();
    ref_pie.eat();
}