use std::collections::HashMap;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use rocket::request::{Form, FromFormValue};
use rocket::http::Cookies;
use chrono::NaiveDateTime;
use mysql::serde::Serialize;

use mysql as my;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Category {
    id: i32,
    slug: String,
    name: String,
    created_at: NaiveDateTime,
}

#[derive(Debug, FromFormValue, Serialize, Deserialize)]
enum Categories {
    A, B, C
}

#[derive(Debug, FromForm)]
pub struct FormInput {
    title: String,
    post: String,
    category: Categories,
}

#[get("/admin/post")]
pub fn new(mut cookies: Cookies) -> Template {
    // let context = HashMap::<&str, &str>::new();
    // let mut context = HashMap::<&str, &str>::new();
    let mut context = HashMap::new();
    let username = cookies.get_private("username");
    // context.insert("username", &username.to_string());

    if let Some(ref username) = username {
        // context.insert("username", username.value());
    }


    ///// Blabla

    // Let's select payments from database

    let mysql = my::Pool::new("mysql://root:test@localhost:3306/train").unwrap();

    let categoryList: Vec<Category> =
    mysql.prep_exec("SELECT * from category", ())
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, slug, name, created_at) = my::from_row(row);
            Category {
                id: id,
                slug: slug,
                name: name,
                created_at: created_at,
            }
        }).collect()
    }).unwrap();
    println!("{:#?}", categoryList);

    context.insert("categories", categoryList);

    Template::render("admin/new", &context)
}

#[post("/admin/post", data = "<post>")]
pub fn add(post: Form<FormInput>) -> Result<Redirect, String> {
    
    println!("{:#?}", post);

    Ok(Redirect::to("/admin/post"))
    /*if user.username == "choppy" {
        if let Ok(StrongPassword("password")) = user.password {
            Ok(Redirect::to("/user/Sergio"))
        } else {
            Err("Wrong password!".to_string())
        }
    } else {
        Err(format!("Unrecognized user, '{}'.", user.username))
    }*/
}
