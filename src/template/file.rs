

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
