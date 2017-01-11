#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(drop_types_in_const)]
#![feature(const_fn)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use r2d2_diesel::ConnectionManager;
use rocket_contrib::JSON;
use dotenv::dotenv;
use std::env;

mod model;
mod schema;

use self::model::{Post, NewPost};
use self::schema::posts;
use self::schema::posts::dsl::*;

lazy_static! {
  static ref POOL: r2d2::Pool<ConnectionManager<PgConnection>> = {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL to be set");
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(config, manager).expect("Failed to create connection pool")
  };
}

macro_rules! get_conn {
    () => (POOL.clone().get().unwrap())
}

#[get("/posts")]
fn index() -> JSON<Vec<Post>> {
  let conn = get_conn!();
  JSON(posts.filter(published.eq(true)).limit(20).load::<Post>(&*conn).expect("Unable to load posts"))
}

#[put("/posts/new", data="<post>")]
fn add_post(post: JSON<NewPost>) -> JSON<Post> {
  let conn = POOL.clone().get().unwrap();
  let new_post = post.unwrap();
  JSON(diesel::insert(&new_post).into(posts::table).get_result(&*conn).expect("Unable to add post"))
}

#[post("/posts/<post_id>/publish")]
fn update_post(post_id: i32) -> JSON<Post> {
  let conn = POOL.clone().get().unwrap();
  JSON(diesel::update(posts.find(post_id)).set(published.eq(true)).get_result(&*conn).expect("Unable to update post"))
}

#[delete("/posts/<post_id>")]
fn delete_post(post_id: i32) {
  let conn = get_conn!();
  diesel::delete(posts.find(post_id)).execute(&*conn).expect("Unable to delete post");
}

fn main() {
  dotenv().ok();

  rocket::ignite().mount("/api", routes![index, add_post, update_post, delete_post]).launch();
}