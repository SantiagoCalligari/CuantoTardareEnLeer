use diesel::prelude::*;
use Rocknance::models::*;
use Rocknance::*;

fn main() {
    use Rocknance::schema::books::dsl::*;
    let connection = &mut establish_connection();
    let titlequery = "Tr3s";
    let results = books
        .filter(title.like(titlequery))
        .limit(7)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");
    println!("Displaying {} posts", results.len());
    for post in results {
        println!("--------------------");
        println!("{}", post.title);
        println!("{}", post.author);
    }
}
