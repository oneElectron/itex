mod target_location;
mod template_url;

use reqwest::blocking::Client;
use zip::ZipArchive;
use std::{
  process::exit, 
  io::Write,
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
  let mut downloaded_file = client.get(template_url::get_template_url())
    .send()
    .expect("Couldn't download templates folder");

  let mut file_in_vec = Vec::new();
  downloaded_file.copy_to(&mut file_in_vec).unwrap();

  let mut archive = ZipArchive::new(std::io::Cursor::new(file_in_vec))
    .expect("could not parse downloaded data"); 
  
  archive.extract(target_location::install_location())
    .expect("could not extract to app data folder");
}
