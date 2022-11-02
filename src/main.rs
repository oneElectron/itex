pub mod template;

fn main() -> std::io::Result<()> {
    // Create out folder
    let path = std::env::current_dir()?;
    let cudir = path.to_str();
    std::fs::create_dir([cudir.unwrap(), "/out"].join("")).expect("printed");

    // print args
    let mut template_name = std::env::args().nth(1);
    if template_name.is_none() {
        template_name = Some(template::ask_for_template_name());
    }
    let template_name = std::string::String::from(template_name.unwrap().replace("\n", ""));
    template::copy_template(template_name);
    

    
    Ok(())
}
