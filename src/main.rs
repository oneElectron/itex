use std::env;
use std::fs;
use console;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    let cudir = path.to_str();
    if fs::create_dir([cudir.unwrap(), "/out"].join("")).is_err(){
        print!("{}", console::style("Failed to create directory").bold().red())
    }
    Ok(())
}
