// SPDX-License-Identifier: MIT

use std::io::Write;

use chrono::Datelike;

fn current_week_number() -> u32 {
    let now = chrono::Utc::now();
    let week = now.iso_week().week();
    week
}

fn filename_for_week(folder: std::path::PathBuf, week: u32) -> String {
    format!("{}/w{}.md", folder.to_str().unwrap(), week)
}

fn add_journal(text: String) {
    let j_path = std::env::var("J_PATH").expect("J_PATH env var not set");
    let path = std::path::Path::new(&j_path);
    let year = chrono::Utc::now().year().to_string();
    let path_with_year = path.join(year);

    if !path_with_year.exists() {
        std::fs::create_dir(&path_with_year).unwrap();
    }

    let filename = filename_for_week(path_with_year, current_week_number());

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();

    file.write_all("- ".as_bytes()).unwrap();
    file.write_all(text.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
}

fn main() {
    let j_path = std::env::var("J_PATH").expect("J_PATH env var not set");
    let path = std::path::Path::new(&j_path);

    if !path.exists() {
        eprintln!("J_PATH does not exist: {}", j_path);
        std::process::exit(1);
    }

    if !path.is_dir() {
        eprintln!("J_PATH is not a directory: {}", j_path);
        std::process::exit(1);
    }

    // bail out if argument wasn't provided:
    if std::env::args().len() < 2 {
        eprintln!("No text provided");
        std::process::exit(1);
    }

    let text = std::env::args().nth(1).unwrap();

    add_journal(text);
}
