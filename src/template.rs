

pub fn copy_template(name:String) {
  // find templates folder
  let mut maintex = std::path::PathBuf::from("/opt/homebrew/Cellar/itex/0.1.1/templates/iSci/main.tex");
  let mainbib = std::path::PathBuf::from("/opt/homebrew/Cellar/itex/0.1.1/templates/iSci/main.bib");
  let makefile = std::path::PathBuf::from("/opt/homebrew/Cellar/itex/0.1.1/templates/iSci/Makefile");

  // find current dir
  let mut pwd = String::new();
  for (key, value) in std::env::vars() {
      if key == "PWD" { pwd = key; }
  }
  let pwd:std::path::PathBuf = std::path::PathBuf::from(pwd.clone());

  // copy template to current dir
  println!("{}", maintex.display());
  println!("{:?}", std::fs::copy(maintex, pwd.with_file_name("main.tex")));
  std::fs::copy(mainbib, pwd.with_file_name("main.bib"));
  std::fs::copy(makefile, pwd.with_file_name("Makefile"));
}
