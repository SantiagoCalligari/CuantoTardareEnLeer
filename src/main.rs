#[macro_use]
extern crate rocket;
use diesel::prelude::*;
use Rocknance::models::*;
use Rocknance::*;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    title: String,
    author: String,
    url_cover: String,
    word_count: i32,
    id: i32,
    language: String,
}


fn formatter(results: Vec<Rocknance::models::Post>) -> String {
    let mut res = format!("");
    for post in results {
        res = format!(
            "{} \n title: {}\n author:{}\n word_count:{}\n\n a 300 palabras por minuto vas a tardar {} horas en leerlo \n",
             res, post.title, post.author, post.word_count, post.word_count/18000
        );
    }
    res
}

fn search_book(book_title: String, book_author:String) -> Vec<Rocknance::models::Post>{
    use Rocknance::schema::books::dsl::*;
    let connection = &mut establish_connection();
    books
        .filter(title.like(format!("%{}%",book_title)))
        .filter(author.like(format!("%{}%",book_author)))
        .limit(10)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts")
}

#[get("/author/<author>")]
async fn author_request(author: String) -> String {
    let results = search_book(String::from(""), author);
    formatter(results)
}

#[get("/title/<title>")]
async fn book_request(title: String) -> String {
    let results = search_book(title, String::from(""));
    formatter(results)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![ book_request, author_request])
}

