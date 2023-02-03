use std::{
    process,
    process::exit
};

fn _add_windows_template_folder() { // TODO
  let mut app_data_dir = std::path::PathBuf::from(std::env::var("LOCALAPPDATA").expect("No App Data dir found"));
  app_data_dir.push("itex");
  if !app_data_dir.is_dir() {
    if std::fs::create_dir(&app_data_dir).is_err() {
      println!("Something went wrong creating a folder in AppData");
      exit(2);
    }
  }
  app_data_dir.push("itex-templates");
  if !app_data_dir.is_dir() {
    app_data_dir.push("itex.zip");
    let _output = process::Command::new("curl")
      .arg("-o")
      .arg(app_data_dir
        .to_str()
        .unwrap()
        .trim())
      .arg("https://github.com/oneelectron/itex/releases/latest/download/");
  }
}
  