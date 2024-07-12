use std::fs::{self, OpenOptions};
use std::io::Write;

use tauri::{App, Manager};

static PASS_FILE_NAME: &str = "dfgs.txt";

pub fn setup_password_file(app: &mut App) {
    let handle = app.handle();
    let app_data_path = handle
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir");
    println!("{:?}", app_data_path);
    if !app_data_path.exists() {
        fs::create_dir(&app_data_path).ok();
    }
    let mut pass_file_path = app_data_path.clone();
    pass_file_path.push(PASS_FILE_NAME);
    if !pass_file_path.exists() {
        let mut pass_file = OpenOptions::new()
            .write(true)
            .create_new(!pass_file_path.exists())
            .open(pass_file_path)
            .expect("Pass file could not be open");
        pass_file
            .write(b"secure_password")
            .expect("Could not write password file");
        pass_file.flush().expect("Failed to flush password file");
    }
}
