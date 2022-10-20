pub fn copy_template(name:String) {
  // find brew
  let output = Command::new("brew")
                     .arg("command")
                     .output()
                     .expect("Failed to execute command");
  // find template folder
  // copy template to current dir
}
