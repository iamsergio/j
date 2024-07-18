// SPDX-License-Identifier: MIT

use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use chrono::Datelike;

fn current_week_number() -> u32 {
    let now = chrono::Utc::now();
    now.iso_week().week()
}

fn filename_for_week(folder: std::path::PathBuf, week: u32) -> String {
    format!("{}/w{}.md", folder.to_str().unwrap(), week)
}

fn add_journal(text: String) {
    let path_with_year = path_with_year();

    if !path_with_year.exists() {
        std::fs::create_dir(&path_with_year).unwrap();
    }

    let week = current_week_number();
    let filename = filename_for_week(path_with_year, week);

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(filename.clone())
        .unwrap();

    file.write_all("- ".as_bytes()).unwrap();
    file.write_all(text.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();

    show_journal(week);
}

fn path_with_year() -> PathBuf {
    let j_path = std::env::var("J_PATH").expect("J_PATH env var not set");
    let path = std::path::Path::new(&j_path);
    let year = chrono::Utc::now().year().to_string();
    let path_with_year = path.join(year);
    path_with_year
}

fn show_journal(week: u32) {
    let path_with_year = path_with_year();

    if !path_with_year.exists() {
        std::fs::create_dir(&path_with_year).unwrap();
    }

    let filename = filename_for_week(path_with_year, week);

    let mut file = File::open(&filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}:\n{}", filename, contents);
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

    // show current journal if no argument provided
    if std::env::args().len() < 2 {
        show_journal(current_week_number());
        std::process::exit(0);
    }

    // join all arguments into a single string:
    let text = std::env::args().skip(1).collect::<Vec<String>>().join(" ");

    add_journal(text);
}
