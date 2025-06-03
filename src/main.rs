#![allow(static_mut_refs)]

use eframe::egui::Color32;
use eframe::egui::Widget;
use full_logger::logger::*;
use full_logger::file_manager::*;

use full_logger::thread::flush_log_thread;
use full_logger::thread::start_log_thread;
use rocket::{get, routes};

#[derive(Debug, Clone, Default)]
struct Log {
    source: String,
    level: String,
    message: String
}

static mut LOGS: Vec::<Log> = Vec::<Log>::new();

#[derive(Default)]
struct WatchedValue {
    name: String,
    value: String
}

static mut WATCHED: Vec::<WatchedValue> = Vec::<WatchedValue>::new();

#[get("/<source>/<level>/<msg>")]
async fn server_log(source: &str, level: &str, msg: &str) -> &'static str {
    simple_log(vec![source, level], msg);

    unsafe {
        let mut log: Log = Log::default();
        log.source = source.into();
        log.level = level.to_string().to_lowercase();
        log.message = msg.into();

        LOGS.push(log);
    }
    
    "Received log"
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
    "https://github.com/eVisualUser/log-server"
}

fn get_logs() -> &'static mut Vec<Log> {
    unsafe {
        &mut LOGS
    }
}

async fn launch() -> Result<(), rocket::Error> {
    set_allow_console_log(true);
    set_or_create_global_log_file("log", FileSize::Mo(100));
    start_log_thread(10, 1);

    let rocket = rocket::build()
        .mount("/", routes![get_portfolio])
        .mount("/log", routes![server_log])
        .mount("/watch", routes![watch_value])
        .configure(rocket::config::Config {
            port: 7300,
            ..rocket::config::Config::default()
        })
        .launch();

    rocket.await?;

    Ok(())
}

fn main() {
    let server_runtime = tokio::runtime::Runtime::new().unwrap();
    let server = server_runtime.spawn(async {
        launch().await
    });

    let options: eframe::NativeOptions = eframe::NativeOptions::default();

    eframe::run_simple_native("Logging Server", options, |ctx, _frame| {
        eframe::egui::SidePanel::left("WatchedValues").resizable(true).show(ctx, |ui|{
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
            .animated(false)
            .auto_shrink(false)
            .min_scrolled_height(32.0)
            .show_rows(ui, ui.text_style_height(&eframe::egui::TextStyle::Body), 20, |ui, _|{
                let mut last_source: Option<String> = None;
                for log in get_logs() {
                    let mut color = Color32::WHITE;
                    let mut text = eframe::egui::RichText::new(log.message.clone());
                    let mut level = eframe::egui::RichText::new(log.level.clone());
                    let mut source = eframe::egui::RichText::new(log.source.clone());

                    if log.level == "error" {
                        color = Color32::RED;
                        text = text.underline();
                        text = text.strong();
                    } else if log.level == "warning" {
                        color = Color32::ORANGE;
                        text = text.underline();
                        text = text.strong();
                    }

                    text = text.color(color);
                    level = level.strong().color(color);
                    source = source.strong().color(color);

                    if last_source.is_some() {
                        if last_source.unwrap() != log.source {
                            ui.separator();
                        }
                    }

                    ui.horizontal(|ui| {
                        ui.label(source);
                        ui.label(level);
                        ui.label(text);
                    });

                    last_source = Some(log.source.clone());                                                                             
                }
            });
        });

        eframe::egui::TopBottomPanel::top("Info").resizable(true).show(ctx, |ui|{
            ui.vertical(|ui|{
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

                ui.hyperlink("http://127.0.0.1:7300");
            });
        });
    }).unwrap();

    flush_log_thread(1);

    server.abort();
}
