
pub fn find_templates_folder() -> std::result::Result<std::path::PathBuf, i32> {
  let cellar_path = std::process::Command::new("brew").arg("--Cellar").output();
  if cellar_path.is_err() {
    println!("failed to find homebrew is your path");
  }
  else {
    let cellar_path = std::string::String::from_utf8(cellar_path.unwrap().stdout.to_vec());
    if cellar_path.is_err() {
      println!("error while decoding homebrew cellar path");
      panic!();
    }
    let cellar_path = cellar_path.unwrap().replace("\n", "");
    let mut path_to_templates = std::path::PathBuf::from(cellar_path);
    
    path_to_templates.push("itex");
    if !path_to_templates.is_dir() {
      println!("itex is not a folder");
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
  }
  return Err(1);
}


pub fn copy_template(name:std::string::String) {
  let mut path_to_templates = find_templates_folder().unwrap();
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
