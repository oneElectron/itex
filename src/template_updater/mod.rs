#[cfg(feature = "updater")]
mod target_location;
#[cfg(feature = "updater")]
mod template_url;

#[cfg(feature = "updater")]
use reqwest::blocking::Client;
#[cfg(feature = "updater")]
use std::{io::Write, process::exit};
#[cfg(feature = "updater")]
use zip::ZipArchive;

#[cfg(feature = "updater")]
pub fn download_templates() {
    let mut input = String::new();
    println!(
        "It looks like the itex-templates folder is not installed, would you like to install it?"
    );
    println!("ITex will install into your AppData folder on Windows");

    print!("(Y/n): ");
    std::io::stdout().flush().expect("flush failed");

    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read from stdin");

    let input = input.trim();

    if input != "y" && input != "Y" && input !="yes" && input != "Yes" {
        println!("Aborting");
        exit(0);
    };

    println!("downloading...");

    let client = Client::new();
    let mut downloaded_file = client
        .get(template_url::get_template_url())
        .send()
        .expect("Couldn't download templates folder");

    let mut file_in_vec = Vec::new();
    downloaded_file.copy_to(&mut file_in_vec).unwrap();

    let mut archive = ZipArchive::new(std::io::Cursor::new(file_in_vec))
        .expect("could not parse downloaded data");

    archive
        .extract(target_location::install_location())
        .expect("could not extract to app data folder");
}

#[cfg(not(feature = "updater"))]
pub fn download_templates() {
    println!("updater is not enabled for this build. If you used a package manager to install itex, the templates are kept up to date by the package manager");
}
