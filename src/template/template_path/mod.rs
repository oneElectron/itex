mod template_path;

pub fn find_templates_folder(debug: bool, disable_os_search:bool) -> std::result::Result<std::path::PathBuf, i32> {
    if !disable_os_search {
      if !cfg!(windows) { // if os is UNIX
        if let Ok(path_to_templates) = template_path::search_in_homebrew(debug) {
          return Ok(path_to_templates);
        }
        return Err(0);
      }
  
      else { // if OS is windows
        if let Ok(path_to_templates) = template_path::search_in_windows() {
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
  