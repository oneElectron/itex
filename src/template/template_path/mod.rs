mod template_path;

use template_path::{search_in_homebrew, search_in_windows};
use std::{
  result::Result,
  env,
};

pub fn find_templates_folder(disable_os_search:bool) -> Result<std::path::PathBuf, i32> {
  if !disable_os_search {
    if !cfg!(windows) { // if OS is UNIX
      if let Ok(path_to_templates) = search_in_homebrew() {
        return Ok(path_to_templates);
      }
      return Err(0);
    }

    else { // if OS is Windows
      if let Ok(path_to_templates) = search_in_windows() {
        return Ok(path_to_templates);
      }
      // add_windows_template_folder();
      return Err(1);    
    }
  }

  // search current directory
  let pwd = env::current_dir();
  let mut pwd = pwd.unwrap();
  pwd.push("itex-templates");
  if pwd.is_dir() {
    return Ok(pwd);
  }
  drop(pwd);

  // search in ..
  let mut previous_dir = env::current_dir().unwrap().parent().unwrap().to_path_buf();
  previous_dir.push("itex-templates");
  if previous_dir.is_dir() {
    return Ok(previous_dir);
  }

  Err(0)
}
  