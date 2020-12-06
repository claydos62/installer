#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use std::{
    fs::File,
    io::prelude::*,
};

use serde::Serialize;

#[derive(Serialize)]
struct Reply {
    data: String,
}

fn main() {
    tauri::AppBuilder::new()
        .setup(|webview, _source| {
            let mut webview = webview.as_mut();
            let mut webview_clone = webview.clone();
            tauri::event::listen(String::from("js-event"), move |msg| {
                println!("Got js-event with message '{:?}'", msg);
                let reply = Reply {
                    data: String::from("something else"),
                };

                tauri::event::emit(
                    &mut webview,
                    String::from("rust-event"),
                    Some(serde_json::to_string(&reply).unwrap()),
                ).expect("failed to emit");
            });

            webview_clone
                .dispatch(move |w| {
                    w.eval("window.onTauriInit()");
                })
                .expect("failed to dispatch");
        })
        .invoke_handler(|_webview, arg| {
            use cmd::Cmd::*;
            match serde_json::from_str(arg) {
                Err(e) => Err(e.to_string()),
                Ok(command) => {
                    match command {
                        DownloadA32NX => {
                            std::thread::spawn(move || {
                                println!("Downloading");
                                let resp = reqwest::blocking::get("https://flybywiresim-packages.nyc3.cdn.digitaloceanspaces.com/vmaster/A32NX-master.zip").unwrap();

                                let mut file = File::create("A32NX-Master.zip").unwrap();
                                file.write_all(&resp.bytes().unwrap()).unwrap();
                            });
                        }
                    }
                    Ok(())
                  }
            }
        })
        .build()
        .run();
}
