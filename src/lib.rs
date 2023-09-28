pub mod enums;
mod winapi;
mod paths;
mod chromium;
mod mozilla;
mod utils;
use std::error::Error;

use chromium::chromium_based;
use mozilla::firefox_based;
use enums::*;



pub fn firefox(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let db_path = paths::find_firefox_paths();
    firefox_based(db_path, domains)
}

pub fn chrome(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let (key, db_path) = paths::find_chrome_paths();
    chromium_based(key, db_path, domains)
}


pub fn brave(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let (key, db_path) = paths::find_brave_paths();
    chromium_based(key, db_path, domains)
}

pub fn edge(domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    let (key, db_path) = paths::find_edge_paths();
    chromium_based(key, db_path, domains)
}

