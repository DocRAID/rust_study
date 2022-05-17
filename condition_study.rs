fn array(){
	let arr = ["hello","goodbye"];
	println!("{}",arr[0]);
	let ann = [1;5];
	println!("{} {}",ann[0],ann[1]);
}

fn vector(){
	let mut fruits = Vec::new();
	fruits.push("apple");
	fruits.push("banana");
	fruits.push("orange");
	println!("{:?}",fruits);
	// fruits.push(1); 안됨
	println!("{:?}",fruits.pop());
}

fn main(){
	array();
	vector();
}

