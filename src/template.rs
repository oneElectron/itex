
pub fn find_templates_folder() {
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
    
    let versions = std::fs::read_dir(&path_to_templates);
    if versions.is_err(){
      println!("failed to read {:?}", path_to_templates.as_os_str());
    }
    let versions = versions.unwrap();
    if !(versions.count() == 1) {
      println!("You have more than more version of itex installed");
      panic!();
    }
    


  }
}

/*
pub fn copy_template(name:&str) {
  let mut path_to_templates = find_templates();

  path_to_templates.push_str("/itex-templates");
  path_to_templates.push_str(name);
  let template_files = std::fs::read_dir(&path_to_templates).unwrap();

  // find current dir
  let mut pwd = String::new();
  for (key, value) in std::env::vars() {
      if key == "PWD" { pwd = value; }
  }
  pwd.push_str("/file.txt");
  let pwd:std::path::PathBuf = std::path::PathBuf::from(pwd);

  // copy template to current dir
  for file in template_files {
    if std::fs::copy(file.as_ref().unwrap().path(), pwd.as_path().with_file_name(file.unwrap().file_name())).is_err() {
      println!("could not copy");
    }
  }
}*/
