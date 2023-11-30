mod models;
mod repositories;
mod schema;

#[rocket::get("/rustaceans")]
fn get_rustaceans() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![get_rustaceans])
        .launch()
        .await;
}
