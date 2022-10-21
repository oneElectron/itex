pub mod template;

fn main() -> std::io::Result<()> {
    // Create out folder
    let path = std::env::current_dir()?;
    let cudir = path.to_str();
    std::fs::create_dir([cudir.unwrap(), "/out"].join("")).expect("printed");

    // pick template
    let template_name = "/iSci";
    //std::io::stdin()
    //                .read_line(&mut template_name)
    //                .expect("Could not read input");

    // copy template
    template::copy_template(template_name);
    
    Ok(())
}
