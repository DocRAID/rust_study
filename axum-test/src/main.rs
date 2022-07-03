use axum::{
    routing::get,
    Router,
};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/home", get(home));

    // run it with hyper on localhost:3000
    let path = "127.0.0.1:3000";
    println!("started on {} !!",path);
    axum::Server::bind(&path.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn root() -> String {
    println!("root called!");
    format!("hello world")
}
async fn home() -> String {
    println!("home called!");
    format!("home router")
}