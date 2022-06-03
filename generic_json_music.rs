trait AsJson{
    fn as_json(&self) -> String;
}

fn send_data_as_json<T:AsJson>(value: &T){
    println!("Sending JSON data to server.....");
    println!("=> {}",value.as_json());
    println!("DONE!");
}

struct Music {
    name: String,
    singer: String,
    lyrics: bool,
}
impl AsJson for Music {
    fn as_json(&self) -> String{
        format!(
            r#"{{ "type": "music", "name": "{}", "singer": "{}", lyrics: {} }}"#,
            self.name, self.singer, self.lyrics
        )
    }
}
fn main(){
    let arrow = Music{
        name: String::from("arrow"),
        singer: String::from("half-alive"),
        lyrics: true,
    };
    send_data_as_json(&arrow);
}