mod search_for_templates;

pub fn copy_template(name:std::string::String, debug: bool, disable_os_search:bool) {
  let path_to_templates = find_templates_folder(debug, disable_os_search);
  if path_to_templates.is_err() {
    println!("{}", console::style("Failed to find templates folder").red().bold())
  }
  let mut path_to_templates = path_to_templates.unwrap();
  path_to_templates.push(name);

  if debug {
    println!("{}", path_to_templates.to_str().unwrap());
  }

  let template_files = std::fs::read_dir(path_to_templates.to_str().unwrap().trim());
  let template_files1 = std::fs::read_dir(path_to_templates.to_str().unwrap().trim());
  if template_files.is_err() {
    println!("Could not find path: {}", path_to_templates.to_str().unwrap());
    panic!();
  }
  let template_files = template_files.unwrap();
  let template_files1 = template_files1.unwrap();

  // find current dir
  let mut pwd = std::env::current_dir().unwrap();

  pwd.push("file.txt");

  let mut exit = false;
  for file in template_files1 {
    let file = file.unwrap().file_name();
    if pwd.with_file_name(&file).exists() {
      println!("file exists: {}, remove this file before running", file.to_str().unwrap());
      exit = true;
    }
  }
  if exit {
    std::process::exit(0);
  }

  // copy template to current dir
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

pub fn find_templates_folder(debug: bool, disable_os_search:bool) -> std::result::Result<std::path::PathBuf, i32> {
  if !disable_os_search {
    if !cfg!(windows) { // if os is UNIX
      if let Ok(path_to_templates) = search_for_templates::search_in_homebrew(debug) {
        return Ok(path_to_templates);
      }
      return Err(0);
    }

    else { // if OS is windows
      if let Ok(path_to_templates) = search_for_templates::search_in_windows() {
        return Ok(path_to_templates);
      }
      //add_windows_template_folder();
      return Err(0);    
    }
  }

  // search current directory
  let pwd = std::env::current_dir();
  let mut pwd = pwd.unwrap();
  pwd.push("itex-templates");
  if pwd.is_dir() {
    return Ok(pwd);
  }
  drop(pwd);

  // search in ..
  let mut previous_dir = std::env::current_dir().unwrap().parent().unwrap().to_path_buf();
  previous_dir.push("itex-templates");
  if previous_dir.is_dir() {
    return Ok(previous_dir);
  }

  Err(0)
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