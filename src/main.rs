pub mod template;

fn main() -> std::io::Result<()> {
    // Create out folder
    let path = std::env::current_dir()?;
    let cudir = path.to_str();
    std::fs::create_dir([cudir.unwrap(), "/out"].join("")).expect("printed");

    // print args
    let template_name = std::env::args().nth(1);
    if template_name.is_none() {
        println!("failed to parse or find template name");
        panic!();
    }
    let template_name = template_name.unwrap();

    // pick template
    let template_name = std::string::String::from(template_name);

    // copy template
    let template_name = template_name.replace("\n", "");
    template::copy_template(template_name);
    
    Ok(())
}
