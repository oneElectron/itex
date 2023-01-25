use std::ffi::OsString;

pub fn search_in_homebrew(debug: bool) -> std::result::Result<std::path::PathBuf, i32> {
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
  let mut cellar_path = std::path::PathBuf::from(cellar_path.unwrap().trim());
  cellar_path.push("itex");
  
  let itex_dir = std::fs::read_dir(&cellar_path);
  if itex_dir.is_err() {
    println!("path: {}", cellar_path.to_str().unwrap());
    println!("itex not found in homebrew");
    panic!();
  }
  let itex_dir = itex_dir.unwrap();
  let mut itex_version_number:OsString = Default::default();
  for dl in itex_dir {
    itex_version_number = dl.unwrap().file_name();
  }
  cellar_path.push(itex_version_number);
  
  if debug {
    println!("cellar path = {}", cellar_path.to_str().unwrap());
  }
  cellar_path.push("itex-templates");

  return Ok(cellar_path);
}

pub fn search_in_windows() -> std::result::Result<std::path::PathBuf, i32> {
  let mut app_data_dir = std::path::PathBuf::from(std::env::var("LOCALAPPDATA").expect("No App Data dir found"));
  app_data_dir.push("itex");
  app_data_dir.push("itex-templates");  
  if app_data_dir.is_dir() {
    return Ok(app_data_dir);
  }
  else {
    return Err(0);
  }
}
