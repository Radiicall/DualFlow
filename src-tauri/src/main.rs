// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hidapi::HidApi;
use once_cell::unsync::Lazy;
use serde::Serialize;
use settings::AppSettings;
use tauri::{Manager, Window};
use std::{sync::Mutex, path::PathBuf, fs, io::{Read, Write, Seek}};
use xdg::BaseDirectories;
use serde_json::{json, Value};
use sysinfo::System;

#[cfg(not(feature = "fake"))]
use objects::Controller;
#[cfg(feature = "fake")]
use fake::Controller;

//mod checksum;
#[cfg(not(feature = "fake"))]
mod objects;
#[cfg(feature = "fake")]
mod fake;
mod settings;

static HID_API: Mutex<Lazy<HidApi>> = Mutex::new(Lazy::new(|| HidApi::new().unwrap()));

static mut CONTROLLER: Mutex<Option<Controller>> = Mutex::new(None);

static CONFIG_DIR: Mutex<Lazy<PathBuf>> = Mutex::new(Lazy::new(|| BaseDirectories::new().unwrap().create_config_directory("dualflow").unwrap()));

#[tauri::command]
fn change_strength(strength: u8, trigger: &str, which: usize) {
    let mut controller = unsafe { CONTROLLER.lock().unwrap().clone().unwrap() };
    if trigger == "right" {
        controller.right_trigger.strength[which] = strength;
    } else {
        controller.left_trigger.strength[which] = strength;
    }
    unsafe { *CONTROLLER.get_mut().unwrap() = Some(controller) }
}

#[tauri::command]
fn change_trigger_mode(mode: &str, trigger: &str) {
    let mut controller = unsafe { CONTROLLER.lock().unwrap().clone().unwrap() };
    if trigger == "right" {
        controller.right_trigger.mode = mode.into();
    } else {
        controller.left_trigger.mode = mode.into();
    }
    unsafe { *CONTROLLER.get_mut().unwrap() = Some(controller) }
}

#[derive(Clone, Serialize)]
struct Payload {
    apps: Vec<String>,
}

#[tauri::command]
fn use_with_program(program: &str) {
    let config_file = get_config_file_path();

    let controller = unsafe { CONTROLLER.lock().unwrap().clone().unwrap() };

    let app_settings = AppSettings {
        right_trigger: controller.right_trigger,
        left_trigger: controller.left_trigger,
    };

    let mut new_program = serde_json::Map::new();

    new_program.insert(program.to_string(), json!(app_settings));

    if let Ok(mut file) = fs::File::options().read(true).write(true).append(false).open(&config_file) {
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let mut intermediate: Value = serde_json::from_str(&content).unwrap();

        let contents_as_json = intermediate.as_object_mut().unwrap();

        contents_as_json.extend(new_program);
        contents_as_json["apps"].as_array_mut().unwrap().push(json!(program));

        file.seek(std::io::SeekFrom::Start(0)).unwrap();
        file.set_len(0).unwrap();
        write!(file, "{}", json!(contents_as_json)).unwrap();
    } else if let Ok(mut file) = fs::File::create(&config_file) {
        let mut new_apps = serde_json::Map::new();

        new_apps.insert("apps".to_string(), json!([program]));

        let mut data = serde_json::Map::new();
        data.extend(new_program);
        data.extend(new_apps);

        write!(file, "{}", json!(data)).unwrap();
    } else {
        println!("Unable to open file: {}", config_file);
    }
}

fn check_programs(window: Window) {
    let mut app_open = false;
    loop {
        let mut last_checked = false;
        let config_file = get_config_file_path();

        match fs::read_to_string(&config_file) {
            Ok(file) => {
                let contents: Value = serde_json::from_str(&file).unwrap();
                let apps = contents["apps"].as_array().unwrap();

                window.emit("apps", Payload {
                    apps: apps.iter().map(|app| app.to_string()).collect()
                }).unwrap();

                for app in apps {
                    let app = app.as_str().unwrap();
                    let s = System::new_all();
                    if s.processes_by_exact_name(app).any(|_| true) {
                        app_open = true;
                        last_checked = true;

                        let mut controller = unsafe {CONTROLLER.lock().unwrap().clone().unwrap()};

                        controller.left_trigger = serde_json::from_value(contents[app]["left_trigger"].clone()).unwrap();
                        controller.right_trigger = serde_json::from_value(contents[app]["right_trigger"].clone()).unwrap();

                        unsafe {*CONTROLLER.get_mut().unwrap() = Some(controller)}
                    }
                }
            },
            Err(_) => ()
        };

        if app_open && !last_checked {
            let mut controller = unsafe {CONTROLLER.lock().unwrap().clone().unwrap()};

            controller.left_trigger.mode = "Off".into();
            controller.right_trigger.mode = "Off".into();

            unsafe {*CONTROLLER.get_mut().unwrap() = Some(controller)}
            app_open = false;
        } 

        unsafe {
            CONTROLLER
                .lock()
                .unwrap()
                .clone()
                .unwrap()
                .write(&HID_API.lock().unwrap())
                .unwrap();
        }
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}

#[tauri::command]
fn delete_program(program: &str) {
    let mut contents_as_json: Value = serde_json::from_str(&fs::read_to_string(get_config_file_path()).unwrap()).unwrap();

    let contents_as_object = contents_as_json.as_object_mut().unwrap();

    contents_as_object.remove(program);

    let apps = contents_as_object["apps"].as_array_mut().unwrap();

    *apps = apps.iter().filter(|app| app.as_str().unwrap() != program).map(|app| app.to_owned()).collect();

    fs::write(get_config_file_path(), serde_json::to_string(&contents_as_object).unwrap()).unwrap();
}

fn main() {
    unsafe { *CONTROLLER.get_mut().unwrap() = Some(Controller::new(&HID_API.lock().unwrap()).unwrap()) }

    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

            std::thread::spawn(|| check_programs(main_window));
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            change_strength,
            change_trigger_mode,
            use_with_program,
            delete_program,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_config_file_path() -> String {
    let config_dir = CONFIG_DIR.lock().unwrap();
    let config_file = config_dir.to_string_lossy().to_string() + "/settings.json";
    drop(config_dir);
    config_file
}
