#[macro_use] extern crate rocket;

use full_logger::logger::*;
use full_logger::file_manager::*;

#[get("/")]
fn index() -> &'static str {
    "Server hosted by eVisualUser."
}

#[get("/")]
fn help() -> &'static str {
    "Server used for simple tasks mostly about logging data."
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, World!"
}

#[get("/<user>")]
async fn register(user: &str) {
    match simple_log(vec!["user", "out"], user) {
        Ok(_) => (),
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

#[get("/<user>")]
async fn unregister(user: &str) {
    match simple_log(vec!["user", "out"], user) {
        Ok(_) => (),
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    set_allow_console_log(true);
    set_or_create_global_log_file("log", FileSize::Mo(100));
    set_message_box_trigger(Some(String::from("error")));

    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/register", routes![register])
        .mount("/unregister", routes![unregister])
        .mount("/hello", routes![hello])
        .mount("/help", routes![help])
        .launch()
        .await?;

    Ok(())
}
