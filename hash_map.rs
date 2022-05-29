fn main(){
    use std::collections::HashMap;
    let mut reviews: HashMap<String, String> = HashMap::new();
    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
    reviews.insert(String::from("Book of name"), String::from("Limdongju"));
    reviews.insert(String::from("League of Leagends"),String::from("Temmo"));
    reviews.insert(String::from("C++ Basic"),String::from("Some body"));

    let book:&str ="Book of name";
    reviews.remove(book);
    println!("\nReview for \'{}\' : {:?}",book,reviews.get(book));
}