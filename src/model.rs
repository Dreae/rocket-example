use super::schema::posts;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name="posts"]
pub struct NewPost {
  pub title: String,
  pub body: String,
}