mod template_path;

use template_path::find_templates_folder;

use super::runtime_helper::Options;

use std::{
  fs,
  env,
  process::exit
};

pub fn copy_template(name: std::string::String, runtime_options: Options) {
  let mut path_to_templates = find_templates_folder(runtime_options.disable_os_search)
    .expect("Failed to find templates folder");

  path_to_templates.push(name);

  if cfg!(debug_assertions) {
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
  let mut pwd = env::current_dir()
    .expect("could not find current directory");

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
  if end { exit(0); }

  let template_files = fs::read_dir(path_to_templates)
    .expect("could not find template folder");

  // copy template to current directory
  for file in template_files {
    if std::fs::copy(file.as_ref().unwrap().path(), pwd.with_file_name(file.unwrap().file_name())).is_err() {
      println!("could not copy");
    }
  }
}


pub fn list_template_names() {
  println!("available template names:");
  for folder in fs::read_dir(find_templates_folder(false).unwrap()).unwrap() {
    println!("    {}", folder.unwrap().file_name().to_str().unwrap());
  }
}

