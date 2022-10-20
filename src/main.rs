use std::env;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    let cudir = path.to_str();
    std::fs::create_dir([cudir.unwrap(), "/out"].join("")).expect("printed");
    drop(path);
    let mut path:String = String::new();

    for (key, value) in env::vars() {
        if key == "PATH" {
            path = value;
        }
    }
    let paths = path.split(":");
    for itpath in paths {
        println!("{}", itpath);
    }
    println!();

    Ok(())
}
