pub mod file;

pub fn copy_template(name:std::string::String) {
  let path_to_templates = file::find_templates_folder();
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
