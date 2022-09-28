fn array_for(){
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds{
        println!("{}",bird);
    }
}
fn rev(){
    for i in (1..=100).rev() {
        print!("{} ",i); //내림차순 반복문
    }
    println!("\n end rev()");
}
fn step(){
    for i in (1..100).step_by(3){ // n씩 증가하는 반복문
        print!("{} ",i);
    }
    println!("\nend step()");
}
fn main(){
    array_for();
    rev();
    step();
    
}