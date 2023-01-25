pub mod template;
pub mod parse;

fn main() -> std::io::Result<()> {
    // parse args
    let opts = parse::parse(std::env::args().collect());

    if opts.list_templates {
        template::list_template_names();
        std::process::exit(0);
    }

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
    template::copy_template(std::string::String::from(opts.template_name.clone().replace("\n", "")));
    
    Ok(())
}
