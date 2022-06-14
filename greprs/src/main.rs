use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
struct Config{
    query:String,
    filename:String,
}
fn parse_config(args: &[String]) -> Config{
    //어디서 오류가 나는지, 수정을 쉽게 하기위해 모듈로 나눈다.
    let query = args[1].clone();
    let filename = args[2].clone();
    Config{query,filename}
}