fn array(){
    let mut array = [0;100];
    
    array[0]=10;
    array[1]=20;
    array[3]=100;
    array[4]=19;
    array[5]=10;
    array[7]=10;
    array[6]=10;
    println!("{:?}",array);

}
fn vector(){
	let mut fruits = Vec::new();
	fruits.push("apple");
	fruits.push("banana");
	fruits.push("orange");
	println!("{:?}",fruits);
	// fruits.push(1); 안됨
	println!("{:?}",fruits.pop());

    println!("{:?}",fruits[1]);
}

fn main(){
	array();
	vector();
}
