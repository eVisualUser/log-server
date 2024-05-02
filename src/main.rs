#[macro_use] extern crate rocket;

use eframe::egui::Color32;
use eframe::egui::Widget;
use full_logger::logger::*;
use full_logger::file_manager::*;

static mut LOGS: Vec::<String> = Vec::<String>::new();

#[derive(Default)]
struct WatchedValue {
    name: String,
    value: String
}

static mut WATCHED: Vec::<WatchedValue> = Vec::<WatchedValue>::new();

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

#[get("/<name>/<value>")]
async fn watch_value(name: &str, value: &str) -> &'static str {
    unsafe {
        let mut already_watched = false;
        for watched in WATCHED.iter_mut() {
            if name == watched.name {
                watched.value = value.into();
                already_watched = true;
            }
        }

        if !already_watched {
            WATCHED.push(WatchedValue { name: name.into(), value: value.into() })
        }
    }

    "Watched"
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

async fn launch() -> Result<(), rocket::Error> {
    set_allow_console_log(true);
    set_or_create_global_log_file("log", FileSize::Mo(100));

    let rocket = rocket::build()
        .mount("/", routes![get_portfolio])
        .mount("/log", routes![server_log])
        .mount("/watch", routes![watch_value])
        .launch();

    rocket.await?;

    Ok(())
}

fn main() {
    let server_runtime = tokio::runtime::Runtime::new().unwrap();
    let server = server_runtime.spawn(async {
        launch().await
    });

    let mut options = eframe::NativeOptions::default();
    options.follow_system_theme = true;
    eframe::run_simple_native("Logging Server", options, |ctx, _frame| {
        eframe::egui::SidePanel::left("WathedValues").resizable(true).show(ctx, |ui|{
            if ui.button("Clear").clicked() {
                unsafe { WATCHED.clear(); }
            }
            
            eframe::egui::scroll_area::ScrollArea::vertical()
            .stick_to_bottom(true)
            .animated(true)
            .auto_shrink(false)
            .min_scrolled_height(128.0)
            .show_rows(ui, ui.text_style_height(&eframe::egui::TextStyle::Body), 20, |ui, _|{
                for watched in unsafe { WATCHED.iter() } {
                    ui.columns(2, |columns|{
                        columns[0].label(eframe::egui::RichText::new(watched.name.clone()).strong());
                        columns[1].label(eframe::egui::RichText::new(watched.value.clone()));
                    });
                }
            });
        });
        
        eframe::egui::TopBottomPanel::bottom("console").resizable(true).show(ctx, |ui|{
            if eframe::egui::Button::new("Clear").ui(ui).clicked() {
                unsafe { LOGS.clear(); }
            }
            
            eframe::egui::scroll_area::ScrollArea::vertical()
            .stick_to_bottom(true)
            .animated(true)
            .auto_shrink(false)
            .min_scrolled_height(128.0)
            .show_rows(ui, ui.text_style_height(&eframe::egui::TextStyle::Body), 20, |ui, _|{
                for log in get_logs() {
                    let mut color = Color32::WHITE;
                    let mut text = eframe::egui::RichText::new(log.clone());

                    if log.contains("error") {
                        color = Color32::RED;
                        text = text.underline();
                        text = text.strong();
                    } else if log.contains("warning") {
                        color = Color32::DEBUG_COLOR;
                        text = text.underline();
                        text = text.strong();
                    }

                    text = text.color(color);

                    ui.label(text);
                }
            });
        });

        eframe::egui::CentralPanel::default().show(ctx, |ui|{
            ui.label(eframe::egui::RichText::new(
                "
                This software offer an alternative way to log.
                You can see all the logs on the bottom panel.
                And you can watch value on the left panel.
                To sync logs or watch a value, you need to do a GET http request.
                The adress is declared by Rocket in the terminal.
                Log request: {URL}/log/{source}/{level}/{message}
                Watch value: {URL}/watch/{label}/{value}
                Also all the logs are backup in ./log/*.log
                "
            ));
        });
    }).unwrap();

    server.abort();
}
