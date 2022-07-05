use std::fmt::format;
use rocket::tokio::time::{sleep, Duration};

#[macro_use] extern crate rocket;
#[get("/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
async fn index() -> String {
    format!("home")
}
#[get("/<seconds>")]
async fn delay(seconds:u64) -> String{
    sleep(Duration::from_secs(seconds)).await;
    format!("delay {} seconds!",seconds)
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![hello])
        .mount("/home",routes![index])
        .mount("/delay",routes![delay])
}