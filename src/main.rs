pub mod template;

fn main() -> std::io::Result<()> {
    // Check to see if you can find templates folder
    if template::find_templates_folder().is_err() {
        println!("could not find templates folder");
        panic!();
    }

    // Create out folder
    let mut out_folder = std::env::current_dir().unwrap();
    out_folder.push("out");
    if !out_folder.is_dir() {
        std::fs::create_dir(out_folder).expect("failed to create out folder");
    }
    
    // print args
    let mut template_name = std::env::args().nth(1);
    if template_name.is_none() {
        template_name = Some(template::ask_for_template_name());
    }
    let template_name = std::string::String::from(template_name.unwrap().replace("\n", ""));
    template::copy_template(template_name);
    
    Ok(())
}
