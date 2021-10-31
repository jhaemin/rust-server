use std::path::PathBuf;

use rocket::tokio::time::{sleep, Duration};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds.", seconds)
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/borrow")]
fn borrow() -> String {
    fn change(some_string: &mut String) {
        some_string.push_str(", world!");
    }

    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    let s1 = String::from("hello");
    let mut s2 = s1; // s1 moved to s2 then s1 is no longer valid after this line

    change(&mut s2);

    format!("{}, world!", s2)
}

#[get("/page/<path..>")]
fn get_page(path: PathBuf) -> String {
    format!("{}", path.as_path().display().to_string())
}

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    rocket::build().mount("/", routes![index, delay, hello, get_page, borrow]).launch().await;
}
