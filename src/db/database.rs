use chrono::NaiveDateTime;
use mysql::serde::Serialize;
use mysql;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    id: i32,
    title: String,
    note: String,
    created_at: NaiveDateTime,
}

fn getConnection() -> mysql::Pool
{
    return mysql::Pool::new("mysql://root:test@localhost:3306/train").unwrap();
}

pub fn posts() -> Vec<Post>
{
    let mysql = getConnection();

    let posts: Vec<Post> =
    mysql.prep_exec("SELECT id, title, note, created_at from post", ())
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, title, note, created_at) = mysql::from_row(row);
            Post {
                id: id,
                title: title,
                note: note,
                created_at: created_at,
            }
        }).collect()
    }).unwrap();
    /*println!("{:#?}", posts);*/

    return posts;
}