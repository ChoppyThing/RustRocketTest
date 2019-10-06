use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket::http::Cookies;

use crate::db::database;
use crate::session::session;

#[derive(Debug, Serialize)]
struct IndexTemplate<'a, 'b> {
    cookie: &'a HashMap<&'a str, &'b str>,
    posts: HashMap<&'a str, Vec<database::Post>>
}

#[get("/")]
pub fn index(mut cookies: Cookies) -> Template {
    let mut context = HashMap::<&str, &str>::new();
    let username = cookies.get_private("username");

    if let Some(ref username) = username {
        context.insert("username", username.value());
    }

    let posts:Vec<database::Post> = database::posts();
    let mut postsData = HashMap::<&str, Vec<database::Post>>::new();
    postsData.insert("posts", posts);


    let data = IndexTemplate {
        cookie: &context,
        posts: postsData
    };
    //println!("{:#?}", data);
    Template::render("index", data)
}

/*fn getPosts() -> */

#[get("/test")]
pub fn index_test(user: crate::user::user::User) -> Template {
    // let context = HashMap::<&str, &str>::new();
    let mut context = HashMap::<&str, &str>::new();
    context.insert("username", &user.0);
    Template::render("index", &context)
}

#[get("/world")]
pub fn world() -> &'static str {
    "Hello, worldssss!"
}
