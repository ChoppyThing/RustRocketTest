use rocket::{Request, State, Data, Response};
use rocket::fairing::{AdHoc, Fairing, Info, Kind};

#[derive(Default)]
pub struct Session {
    id: i32,
    username: String,
    timeout: i32,
}

impl Fairing for Session {
    fn info(&self) -> Info {
        Info {
            name: "Session Handler",
            kind: Kind::Request | Kind::Response
        }
    }

    fn on_request(&self, request: &mut Request<'_>, _: &Data) {
        println!("Session: Requesteeeeed");
        println!("Test : {:#?}", request.cookies().get_private("username"));
        let username = request.cookies().get_private("username");

        match &username {
            Some(username) => {
                println!("{:?}", username);

                ()
            },
            _ => {
                println!("Not connected");

                ()
            }
        }

        println!("Blablabla{:?}", username);
    }

    fn on_response(&self, request: &Request<'_>, response: &mut Response<'_>) {
        println!("Session: Responded");
    }
}
