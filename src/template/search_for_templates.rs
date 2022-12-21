pub fn search_in_homebrew() -> std::result::Result<std::path::PathBuf, i32> {
  let output = std::process::Command::new("brew").arg("-v").output();
  if output.is_err() {
    return Err(0);
  }
  drop(output);
  let cellar_path = std::string::String::from_utf8(std::process::Command::new("brew").arg("--Cellar").output().unwrap().stdout.to_vec());
  if cellar_path.is_err() {
    eprintln!("Failed to run brew --Cellar and read the output");
    return Err(0);
  }
  let mut cellar_path = std::path::PathBuf::from(cellar_path.unwrap());
  cellar_path.push("itex");
  println!("cellar path = {}", cellar_path.to_str().unwrap());
  cellar_path.push("itex-templates");

  return Ok(cellar_path);
}

pub fn search_in_windows() -> std::result::Result<std::path::PathBuf, i32> {
  let mut app_data_dir = std::path::PathBuf::from(std::env::var("APP_DATA").expect("No App Data dir found"));
  app_data_dir.push("Local");
  app_data_dir.push("itex");
  app_data_dir.push("itex-templates");  
  if app_data_dir.is_dir() {
    return Ok(app_data_dir);
  }
  else {
    return Err(0);
  }
}
