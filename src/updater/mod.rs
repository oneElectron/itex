#![cfg(feature = "updater")]

mod target_location;
mod template_url;

use super::exit;
use console::style;
use std::io::Write;
use std::path::Path;

pub fn download_templates(ask: bool, extra_info: bool) {
    if extra_info {
        println!("This updater can only update the templates itex uses.");
        println!("To update ITex itself please go to: https://github.com/oneElectron/itex/releases/latest/");
    }
    let mut input = std::string::String::new();

    if ask {
        println!("It looks like the itex-templates folder is not installed or outdated, would you like to install it?");
        println!("ITex will install into your AppData folder on Windows");

        print!("{} ", style("(Y/n):").green());
        std::io::stdout().flush().expect("flush failed");

        std::io::stdin().read_line(&mut input).expect("could not read from stdin");

        let input = input.trim();

        if input != "y" && input != "Y" && input != "yes" && input != "Yes" {
            println!("{}", style("Aborting").red());
            exit!(0);
        };
    }

    println!("downloading...");

    let client = reqwest::blocking::Client::new();
    let mut downloaded_file = client
        .get(template_url::get_template_url())
        .send()
        .expect("Couldn't download templates folder");

    let mut file_in_vec = Vec::new();
    downloaded_file.copy_to(&mut file_in_vec).unwrap();

    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(file_in_vec)).expect("could not parse downloaded data");

    let output_folder = target_location::install_location();
    if output_folder.exists() {
        std::fs::remove_dir_all(output_folder.clone()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    std::fs::create_dir(output_folder.clone()).unwrap();

    archive.extract(output_folder).expect("could not extract to app data folder");
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct TemplatesVersion {
    last_updated: String,
}

pub fn version_check(templates_path: &Path) {
    let mut templates_path = templates_path.to_owned();
    templates_path.push("itex-version-info.toml");

    if !templates_path.is_file() {
        if templates_path.exists() {
            panic!("templates version info exists but is not a file");
        }

        let itex_version_info = format!("last_updated = \"{}\"\n", clap::crate_version!());
        std::fs::write(&templates_path, itex_version_info).unwrap();
    }

    let c = std::fs::read_to_string(templates_path).unwrap();

    let value: TemplatesVersion = toml::from_str(&c).unwrap();

    if value.last_updated.trim() != clap::crate_version!() {
        println!("Templates are out of date");
        download_templates(true, false);
    }
}

pub fn remove_templates() {
    let target = target_location::install_location();
    if target.exists() {
        std::fs::remove_dir_all(target).expect("failed to remove itex-templates folder");
    }
}
