#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use std::{
    path,
    fs,
    io::{self, Write},
    thread,
};

use serde::Serialize;

use walkdir::WalkDir;

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
                            thread::spawn(move || {
                                println!("Downloading");

                                let mut search_path: Option<path::PathBuf> = None;

                                let mut steam_search_path = dirs::data_dir().unwrap();
                                steam_search_path.push("Microsoft Flight Simulator");
                                steam_search_path.push("UserCfg.opt");

                                let mut ms_store_search_path = dirs::data_local_dir().unwrap();
                                ms_store_search_path.push("Packages");
                                ms_store_search_path.push("Microsoft.FlightSimulator_8wekyb3d8bbwe");
                                ms_store_search_path.push("LocalCache");
                                ms_store_search_path.push("UserCfg.opt");

                                if steam_search_path.exists() {
                                    search_path = Some(steam_search_path);
                                } else if ms_store_search_path.exists() {
                                    search_path = Some(ms_store_search_path);
                                } else {
                                    let walker = WalkDir::new(dirs::data_dir().unwrap().parent().unwrap());

                                    for i in walker.into_iter().filter_map(|e| {
                                        match e {
                                            Ok(e) => {
                                                if e.path().to_str().unwrap().contains("Flight") && e.path().to_str().unwrap().contains("UserCfg.opt") {
                                                    Some(e)
                                                } else {
                                                    None
                                                }
                                            }
                                            Err(_) => None,
                                        }
                                    }) {
                                        search_path = Some(path::PathBuf::from(i.path().to_str().unwrap()));
                                        break;
                                    };
                                }

                                println!("{:?}", &search_path);

                                match search_path {
                                    None => println!("MSFS not found"),
                                    Some(_) => {
                                        let contents = fs::read_to_string(search_path.unwrap()).unwrap();

                                        for i in contents.lines() {
                                            if i.contains("InstalledPackagesPath") {
                                                let split_line: Vec<&str> = i.split(" ").collect();
                                                let msfs_community_path = split_line[1];


                                                // Strips the quotes off the path so its parsed correctly
                                                let mut dir = path::PathBuf::from(&msfs_community_path[1..msfs_community_path.len() - 1]);
                                                dir.push("Community");
                                                println!("{:?}", dir);

                                                if dir.exists() {
                                                    let mut orig_a32nx = path::PathBuf::from(&dir);
                                                    orig_a32nx.push("A32NX");

                                                    if orig_a32nx.exists() {
                                                        fs::remove_dir_all(orig_a32nx);
                                                    }

                                                    let resp = reqwest::blocking::get("https://flybywiresim-packages.nyc3.cdn.digitaloceanspaces.com/vmaster/A32NX-master.zip").unwrap();

                                                    let mut file = fs::File::create("a32nx-temp.zip").unwrap();
                                                    file.write_all(&resp.bytes().unwrap()).unwrap();
                                                    drop(file);

                                                    let file = fs::File::open("a32nx-temp.zip").unwrap();
                                                    let mut a32nx_zip = zip::read::ZipArchive::new(file).expect("Failed to read zip");

                                                    for i in 0..a32nx_zip.len() {
                                                        let mut file = a32nx_zip.by_index(i).unwrap();
                                                        let out_path: path::PathBuf = [&dir.as_path(), &path::Path::new(file.name())].iter().collect();

                                                        println!("{:?}", out_path);

                                                        if file.is_dir() {
                                                            fs::create_dir_all(out_path).unwrap();
                                                        } else {
                                                            if let Some(p) = out_path.parent() {
                                                                if !p.exists() {
                                                                    fs::create_dir_all(&p).unwrap();
                                                                }
                                                            }
                                                            let mut out_file = fs::File::create(out_path).unwrap();
                                                            io::copy(&mut file, &mut out_file).unwrap();
                                                        }
                                                    }

                                                    println!("Finished Unzipping");

                                                    fs::remove_file("a32nx-temp.zip").unwrap();
                                                }
                                            }
                                        }
                                    }
                                }
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
