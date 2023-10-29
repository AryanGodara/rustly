#[macro_use] extern crate rocket;

// use rocket::http::{Header, Status};
// use rocket::response::Response;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]     // <- route attribute
fn hello(name: &str) -> String { // <- request handler
    format!("Hello, {}!", name)
    // let mut response = Response::new()
    // response = (format!("Hello, {}!", name));
    // response.set_header(Header::new("X-Test", "value"));
    // response.set_status(Status::Ok);
    // response
}

use rocket::tokio::time::{sleep, Duration};

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited {} seconds", seconds)
}

//? Convert a synchronous function to an asynchronous function
use std::io;
use rocket::tokio::task::spawn_blocking;

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    let vec = spawn_blocking(|| std::fs::read("data.txt")).await
    .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;
    
    Ok(vec)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello])  // /hello/<name> ==> /hello/<name> and / ==> /
        .mount("/hi", routes![hello]) // /hi -> /hello/<name> ==> /hi/hello/<name>
        .mount("/delay", routes![delay]) // /delay/delay/<seconds> ==> /delay/<seconds>
}

// Alternative
// #[rocket::main]
// async fn main() -> Result<(), rocket::Error> {
//     let _rocket = rocket::build()
//         .mount("/", routes![index, hello])  // /hello/<name> ==> /hello/<name> and / ==> /
//         .mount("/hi", routes![hello]) // /hi -> /hello/<name> ==> /hi/hello/<name>
//         .launch()
//         .await?;

//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::http::{Status, Header};

    #[test]
    fn test_index() {
        let client = Client::untracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("Hello, world!".into()));
    }

    #[test]
    fn test_hello() {
        let client = Client::untracked(rocket()).expect("valid rocket instance");
        let response = client.get("/hello/Aryan").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("Hello, Aryan!".into()));
    }

    #[test]
    fn test_hello_with_header() {
        let client = Client::untracked(rocket()).expect("valid rocket instance");
        let response = client.get("/hello/John")
            .header(Header::new("X-Test", "value"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.headers().get_one("X-Test"), Some("value"));
        assert_eq!(response.into_string(), Some("Hello, John!".into()));
    }
}
