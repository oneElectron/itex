use super::exit;
use super::settings;
use console::style;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::str;

pub fn build(debug: bool, project_path: PathBuf) {
    let build_settings = settings::find_and_parse_toml(project_path.clone());
    let mut tex_filename = project_path.clone();
    tex_filename.push(build_settings.tex_filename());
    let tex_filename = tex_filename.as_os_str().to_str().unwrap();
    let output_directory = pdflatex_output_dir(project_path.clone());

    let mut aux_path = project_path.clone();
    aux_path.push("out/main.aux");
    if project_path.is_absolute() {
        let aux_path_without_prefix = aux_path.strip_prefix(std::env::current_dir().unwrap()); // This must be here in order for bibtex to work
        if aux_path_without_prefix.is_err() {
            println!(
                "{}",
                style("The project directory must be a child of the current directory").red().bold()
            );
            exit!(0);
        }
        aux_path = aux_path_without_prefix.unwrap().to_path_buf();
    }

    let pdflatex_args = vec![output_directory.as_str(), tex_filename];
    let bibtex_args = vec![aux_path.to_str().unwrap()];

    let output = Command::new("pdflatex").args(pdflatex_args.clone()).output();

    if output.is_err() {
        println!("{}", style("Error building pdf. Do you have pdflatex installed?").red().bold());
    }

    let output = output.unwrap();

    if build_settings.compile_bib(Some(project_path.clone())) {
        let output = Command::new("bibtex").args(bibtex_args.clone()).output();
        if output.is_err() {
            println!("{}", style("Error building pdf. Do you have bibtex installed?").red().bold());
        }
        if debug {
            let output = output.unwrap();
            let output_stderr = output.clone().stderr;
            print!("{}", std::str::from_utf8(output_stderr.as_slice()).unwrap());

            let output_stdout = output.stdout;
            print!("{}", std::str::from_utf8(output_stdout.as_slice()).unwrap());
        }

        let _ = Command::new("pdflatex").args(pdflatex_args.clone()).output().unwrap();

        let _ = Command::new("pdflatex").args(pdflatex_args).output().unwrap();
    }

    if debug {
        let output = std::str::from_utf8(&output.stdout).unwrap();
        print!("\n{}", output);
    } else {
        remove_files(project_path);
    }
}

pub fn count(project_path: PathBuf) {
    let build_settings = settings::find_and_parse_toml(project_path);

    let tex_file = build_settings.tex_filename();

    let args = vec![tex_file.as_str()];

    let output = Command::new("texcount").args(args).output();

    if output.is_err() {
        println!("{}", style("Error running Texcount. Do you have texcount installed?").red().bold());
    }

    let output = output.expect("Could not run texcount").stdout;

    let output = String::from_utf8(output).unwrap();

    print!("{}", output);
}

pub fn remove_files(project_path: PathBuf) {
    let mut out_folder_path = project_path;
    out_folder_path.push("out");
    if !out_folder_path.is_dir() {
        println!("{}", style("could not find out dir").red().bold());
        exit!(1);
    }

    let out_folder_path = out_folder_path.read_dir().expect("Could not read dir");

    for file in out_folder_path {
        let path = file.unwrap();
        let filename = path.file_name();
        let filename = filename.to_str().unwrap();
        let path = path.path();

        if !ignore_file(filename) && path.is_file() && std::fs::remove_file(path).is_err() {
            println!("{}", style("failed to remove file in out folder").red().bold());
        }
    }
}

fn ignore_file(filename: &str) -> bool {
    let binding = PathBuf::from(filename);
    let extension = binding.extension().unwrap().to_str().unwrap();

    if extension == "pdf" {
        return true;
    }
    false
}

fn pdflatex_output_dir(path: PathBuf) -> String {
    let mut path = path;
    path.push("out");

    let mut output = Vec::new();
    write!(&mut output, "-output-directory={}", path.to_str().unwrap()).unwrap();

    std::str::from_utf8(&output).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignore_file() {
        assert_eq!(super::ignore_file("main.pdf"), true);
        assert_eq!(super::ignore_file("main.log"), false);
    }

    #[test]
    #[ignore = "texlive"]
    fn build_with_bibfile() {
        let mut project_path = PathBuf::from("test_resources/test_cases/builder/build_with_bibfile");
        build(false, project_path.clone());

        project_path.push("out/texput.log");
        assert!(!project_path.is_file());
        assert!(project_path.with_file_name("main.pdf").is_file());
        assert!(!project_path.with_file_name("main.log").is_file());

        std::fs::remove_file(project_path.with_file_name("main.pdf")).expect("could not remove main.pdf");
    }
}
