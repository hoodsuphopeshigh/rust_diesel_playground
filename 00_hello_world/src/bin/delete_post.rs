extern crate diesel;
extern crate hello_world;

use self::diesel::prelude::*;
use self::hello_world::*;
use std::env::args;

fn main() {
    use hello_world::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
