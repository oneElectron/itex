mod template_path;

use template_path::find_templates_folder;

use super::runtime_helper::Options;

use std::{
  process::exit,
  fs,
  env
};

pub fn copy_template(name: std::string::String, runtime_options: Options) {
  let mut path_to_templates = find_templates_folder(runtime_options.debug, runtime_options.disable_os_search)
    .expect("Failed to find templates folder"); 

  path_to_templates.push(name);

  if runtime_options.debug {
    println!("{}", path_to_templates.to_str().unwrap());
  }
  if !path_to_templates.is_dir() {
    println!("could not find a template with the name provided");
    println!("use itex --list to get a list of available templates");
    exit(1);
  }

  let path_to_templates = path_to_templates
  .to_str()
  .unwrap()
  .trim();

  let template_files = fs::read_dir(path_to_templates)
    .expect("could not find template folder");

  // find current dir
  let mut pwd = env::current_dir().expect("could not find current directory");

  pwd.push("file.txt");

  // dry run: find any files in the current folder that will conflict with the template files
  let mut end = false;
  for file in template_files {
    let file = file.unwrap().file_name();
    if pwd.with_file_name(&file).exists() {
      println!("file exists: {}, remove this file before running", file.to_str().unwrap());
      end = true;
    }
  }
  if end {
    exit(0);
  }

  let template_files = fs::read_dir(path_to_templates)
    .expect("could not find template folder");
  // copy template to current directory
  for file in template_files {
    if std::fs::copy(file.as_ref().unwrap().path(), pwd.with_file_name(file.unwrap().file_name())).is_err() {
      println!("could not copy");
    }
  }
}


pub fn list_template_names(debug: bool) {
  println!("available template names:");
  for folder in std::fs::read_dir(find_templates_folder(debug, false).unwrap()).unwrap() {
    println!("    {}", console::style(folder.unwrap().file_name().to_str().unwrap()).blue());
  }
}


fn _add_windows_template_folder() { // TODO
  let mut app_data_dir = std::path::PathBuf::from(std::env::var("LOCALAPPDATA").expect("No App Data dir found"));
  app_data_dir.push("itex");
  if !app_data_dir.is_dir() {
    if std::fs::create_dir(&app_data_dir).is_err() {
      println!("{}", console::style("Something went wrong creating a folder in AppData").red().bold());
      panic!();
    }
  }
  app_data_dir.push("itex-templates");
  if !app_data_dir.is_dir() {
    app_data_dir.push("itex.zip");
    let _output = std::process::Command::new("curl").arg("-o").arg(app_data_dir.to_str().unwrap().trim()).arg("https://github.com/oneelectron/itex/releases/latest/download/");
    
  }
}
