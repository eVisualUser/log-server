#[macro_use] extern crate rocket;

use full_logger::logger::*;
use full_logger::file_manager::*;

static mut LOGS: Vec::<String> = Vec::<String>::new();

#[get("/<source>/<level>/<msg>")]
async fn server_log(source: &str, level: &str, msg: &str) -> &'static str {
    match simple_log(vec![source, level], msg) {
        Ok(_) => {
            unsafe {
                LOGS.push(format!("{}: {} -> {}", source, level, msg));
            }
            "Succeed to log"
        }
        Err(error) => {
            println!("Error: {}", error);
            "Failed to log"
        }
    }
}

#[get("/")]
async fn get_portfolio() -> &'static str {
    "https://evisualuser.github.io/"
}

fn get_logs() -> &'static mut Vec<String> {
    unsafe {
        &mut LOGS
    }
}

fn get_logs_to_string() -> String {
    let mut result = String::new();
    for log in get_logs() {
        result += log;
        result += "\n";
    }
    return result;
}

async fn launch() -> Result<(), rocket::Error> {
    set_allow_console_log(true);
    set_or_create_global_log_file("log", FileSize::Mo(100));
    set_message_box_trigger(Some(String::from("error")));

    let rocket = rocket::build()
        .mount("/", routes![get_portfolio])
        .mount("/log", routes![server_log])
        .launch();

    rocket.await?;

    Ok(())
}

fn main() {
    let server_runtime = tokio::runtime::Runtime::new().unwrap();
    let server = server_runtime.spawn(async {
        launch().await
    });

    let options = eframe::NativeOptions::default();
    eframe::run_simple_native("Logging Server", options, |ctx, frame| {
        eframe::egui::CentralPanel::default().show(ctx, |ui|{
            eframe::egui::scroll_area::ScrollArea::vertical()
            .animated(true)
            .show_rows(ui, ui.text_style_height(&eframe::egui::TextStyle::Body), 20, |ui, _|{
                ui.label(get_logs_to_string());
            });
        });
    }).unwrap();

    server.abort();
}
