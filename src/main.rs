#[macro_use] extern crate rocket;

//use rocket::tokio::time::{sleep, Duration};

#[get("/hola/<name>")]
async fn delay(name: String ) -> String {
    format!("Hola! {}, que te trae por acá?", name)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, delay])}
