use std::io::Write;

pub fn copy_template(name:std::string::String) {
  let path_to_templates = find_templates_folder();
  if path_to_templates.is_err() {
    println!("{}", console::style("Failed to find templates folder").red().bold())
  }
  let mut path_to_templates = path_to_templates.unwrap();
  path_to_templates.push(name);

  println!("{:?}", &path_to_templates);

  let template_files = std::fs::read_dir(&path_to_templates).unwrap();

  // find current dir
  let mut pwd:Option<std::path::PathBuf> = None;
  for (key, value) in std::env::vars() {
      if key == "PWD" { 
        pwd = Some(std::path::PathBuf::from(value));
      }
  }
  if !pwd.is_some() {
    println!("could not find PWD");
    panic!();
  }

  let mut pwd = pwd.unwrap();
  pwd.push("file.txt");

  // copy template to current dir
  for file in template_files {
    if std::fs::copy(file.as_ref().unwrap().path(), pwd.with_file_name(file.unwrap().file_name())).is_err() {
      println!("could not copy");
    }
  }
}

pub fn ask_for_template_name() -> std::string::String {
  let mut input = std::string::String::new();
  println!("{}", console::style("avaliable template names:").cyan());
  for folder in std::fs::read_dir(find_templates_folder().unwrap()).unwrap() {
    println!("{}", console::style(folder.unwrap().file_name().as_os_str().to_string_lossy()).bold());
  }

  print!("Enter template name: ");
  std::io::stdout().flush().expect("failed to flush");
  let result = std::io::stdin().read_line(&mut input);
  if result.is_err() {
    panic!();
  }

  return input;
}

pub fn find_templates_folder() -> std::result::Result<std::path::PathBuf, i32> {
  // search current directory
  let pwd = std::env::current_dir();
  let mut pwd = pwd.unwrap();
  pwd.push("itex-templates");
  if pwd.is_dir() {
    return Ok(pwd);
  }
  drop(pwd);

  // search in ../
  let pwd = std::env::current_dir();
  let pwd = pwd.unwrap();
  let mut previous_dir = std::path::PathBuf::from(pwd.parent().unwrap());
  previous_dir.push("itex-templates");
  if pwd.is_dir() {
    return Ok(previous_dir);
  }
  drop(pwd);

  // find if installed with homebrew
  let cellar_path = std::process::Command::new("brew").arg("--Cellar").output();

  if !cellar_path.is_err() {
    let cellar_path = std::string::String::from_utf8(cellar_path.unwrap().stdout.to_vec());
    if cellar_path.is_err() {
      println!("error while decoding homebrew cellar path");
      panic!();
    }
    let cellar_path = cellar_path.unwrap().replace("\n", "");
    let mut path_to_templates = std::path::PathBuf::from(cellar_path);
    
    path_to_templates.push("itex");
    if !path_to_templates.is_dir() {
      println!("{}", console::style("itex is not a folder in homebrew's cellar").red().bold());
      panic!();
    }
    
    let versions = std::fs::read_dir(&path_to_templates);
    if versions.is_err(){
      println!("failed to read {:?}", path_to_templates.as_os_str());
    }
    let versions = versions.unwrap();
    if versions.count() != 1 {
      println!("You have more than more version of itex installed");
      panic!();
    }
    let versions = std::fs::read_dir(&path_to_templates).unwrap();
    for version in versions {
      path_to_templates.push(version.unwrap().file_name());
    }
    path_to_templates.push("itex-templates");

    return Ok(path_to_templates);
  } // end if homebrew is found

  return Err(1);
}
