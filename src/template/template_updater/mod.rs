mod target_location;

use reqwest::blocking::Client;
use zip::ZipArchive;
use std::{
  fs::{
    File,
    read_dir
  },
  process::exit, 
  io::Write, iter::Zip
};

pub fn download_templates() {
  let mut input = String::new();
  println!("It looks like the itex-templates folder is not installed, would you like to install it?");
  println!("ITex will install into your AppData folder on Windows");

  print!("(Y/n): ");
  std::io::stdout().flush().expect("flush failed");

  std::io::stdin()
    .read_line(&mut input)
    .expect("could not read from stdin");

  let input = input.trim();

  if input != "y".to_string() {
    println!("Aborting");
    exit(0);
  };

  println!("downloading...");

  let client = Client::new();
  let mut downloaded_file = client.get("https://github.com/oneElectron/itex/archive/refs/tags/v1.0.1.zip")
    .send()
    .expect("Couldn't download templates folder");

  let mut file_in_vec = Vec::new();
  downloaded_file.copy_to(&mut file_in_vec).unwrap();

  let mut archive = ZipArchive::new(std::io::Cursor::new(file_in_vec))
    .expect("could not parse downloaded data"); 
  
  archive.extract(target_location::install_location())
    .expect("could not extract to app data folder");
  
  // move correctly
  let itex_folder = target_location::itex_app_data_folder();
  let contents_itex_folder = read_dir(itex_folder).unwrap();

  todo!();
}
