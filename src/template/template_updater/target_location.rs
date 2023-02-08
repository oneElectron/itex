pub fn install_location() -> std::path::PathBuf {
  let local_app_data = itex_app_data_folder();
  if !local_app_data.is_dir() {
    todo!();
  }

  local_app_data
}

pub fn itex_app_data_folder() -> std::path::PathBuf {
  let local_app_data = std::env::var("LOCALAPPDATA")
    .expect("could not find local app data folder");

  let mut local_app_data = std::path::PathBuf::from(local_app_data);
  local_app_data.push("itex");
  local_app_data
}
