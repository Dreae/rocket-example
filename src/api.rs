use diesel;
use diesel::prelude::*;
use rocket_contrib::JSON;

use super::model::{Post, NewPost};
use super::schema::posts;
use super::schema::posts::dsl::*;
use super::middleware::DBConnection;

#[get("/posts")]
fn index(conn: DBConnection) -> JSON<Vec<Post>> {
  JSON(posts.filter(published.eq(true)).limit(20).load::<Post>(&*conn).expect("Unable to load posts"))
}

#[put("/posts/new", data="<post>")]
fn add_post(conn: DBConnection, post: JSON<NewPost>) -> JSON<Post> {
  let new_post = post.unwrap();
  JSON(diesel::insert(&new_post).into(posts::table).get_result(&*conn).expect("Unable to add post"))
}

#[post("/posts/<post_id>/publish")]
fn update_post(conn: DBConnection, post_id: i32) -> JSON<Post> {
  JSON(diesel::update(posts.find(post_id)).set(published.eq(true)).get_result(&*conn).expect("Unable to update post"))
}

#[delete("/posts/<post_id>")]
fn delete_post(conn: DBConnection, post_id: i32) {
  diesel::delete(posts.find(post_id)).execute(&*conn).expect("Unable to delete post");
}
