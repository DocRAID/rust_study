trait AsJson{
    fn as_json(&self) -> String;
}
fn send_data_as_json<T: AsJson>(value: &T){
    println!("Sending JSON data to server..");
    println!("-> {}", value.as_json());
    println!("Done!\n");
}
struct Person {
    name: String,
    age: u8,
    favorite: String,
}

struct Dog {
    name: String,
    color: String,
    likes_petting: bool,
}

struct Cat {
    name: String,
    sharp_claws: bool,
}

impl AsJson for Person {
    fn as_json(&self) -> String {
	    format!(
	        r#"{{ "type": "person", "name": "{}", "age": {}, "favorite": "{}" }}"#,
	        self.name, self.age, self.favorite
	    )
    }
}

impl AsJson for Dog {
    fn as_json(&self) -> String {
	    format!(
	        r#"{{ "type": "dog", "name": "{}", "color": "{}", "likesPetting": {} }}"#,
	        self.name, self.color, self.likes_petting
	    )
    }
}
impl AsJson for Cat {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "cat", "name": "{}", "charp_claws": {} }}"#,
            self.name, self.sharp_claws
        )
    }
}
fn main(){
    let student = Person{
        name: String::from("Limdongju"),
        age:18,
        favorite: String::from("Alcohol")
    };
    let 김강현 = Dog {
        name: String::from("김강현"),
        color: String::from("black"),
        likes_petting: true,
    };
    let kitty = Cat {
        name: String::from("Kitty"),
        sharp_claws: false,
    };

    send_data_as_json(&student);
    send_data_as_json(&김강현);
    send_data_as_json(&kitty);
}