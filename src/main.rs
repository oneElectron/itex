
fn main() -> std::io::Result<()> {
    // Create out folder
    //let path = std::env::current_dir()?;
    //let cudir = path.to_str();
    //std::fs::create_dir([cudir.unwrap(), "/out"].join("")).expect("printed");

    // Print all env variables to stdout
    let mut path:String = String::new();
    for (key, value) in std::env::vars() {
        println!("{} = {}", key, value);
        if key == "PATH" {
            path = value;
        }
    }
    let paths = path.split(":");
    for itpath in paths {
        println!("{}", itpath);
    }



    
    Ok(())
}
