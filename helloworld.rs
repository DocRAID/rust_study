use std::io::stdin;

fn main() {
	//Declare a mutable input string
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
    	.ok()
        .expect("Failed to read line");
    let x = 10 * 5;
    println!("{} and input string {}",x,input_string);
}