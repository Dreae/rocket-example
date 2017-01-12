use diesel;
use diesel::prelude::*;
use rocket_contrib::JSON;

use super::POOL;
use super::model::{Post, NewPost};
use super::schema::posts;
use super::schema::posts::dsl::*;

#[get("/posts")]
fn index() -> JSON<Vec<Post>> {
  let conn = get_conn!();
  JSON(posts.filter(published.eq(true)).limit(20).load::<Post>(&*conn).expect("Unable to load posts"))
}

#[put("/posts/new", data="<post>")]
fn add_post(post: JSON<NewPost>) -> JSON<Post> {
  let conn = get_conn!();
  let new_post = post.unwrap();
  JSON(diesel::insert(&new_post).into(posts::table).get_result(&*conn).expect("Unable to add post"))
}

#[post("/posts/<post_id>/publish")]
fn update_post(post_id: i32) -> JSON<Post> {
  let conn = get_conn!();
  JSON(diesel::update(posts.find(post_id)).set(published.eq(true)).get_result(&*conn).expect("Unable to update post"))
}

#[delete("/posts/<post_id>")]
fn delete_post(post_id: i32) {
  let conn = get_conn!();
  diesel::delete(posts.find(post_id)).execute(&*conn).expect("Unable to delete post");
}
