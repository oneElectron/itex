

pub fn copy_template(name:&str) {
  // find the Cellar
  let cellar_path = std::process::Command::new("brew").arg("--Cellar").output();
  if cellar_path.is_err() {
    println!("Something went wrong finding the Cellar");
    panic!();
  }

  // let out equal path to Cellar/itex/{version}/templates/{template name}
  let path_to_templates = std::string::String::from_utf8(cellar_path.unwrap().stdout.to_vec()).unwrap();
  let mut path_to_templates = std::string::String::from(path_to_templates.trim_end());
  path_to_templates.push_str("/itex");
  let path_to_templates = std::path::PathBuf::from(path_to_templates);
  let versions = std::fs::read_dir(path_to_templates).unwrap();
  let mut path_to_templates = std::string::String::new();
  for version in versions {
    path_to_templates = std::string::String::from(version.unwrap().path().to_str().unwrap());
  }
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
}
