fn array_for(){
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds{
        println!("{}",bird);
    }
}
fn for_loof(){
    for i in 0..100000{
        println!("{}",i);
    }
}
fn main(){
    array_for();
}