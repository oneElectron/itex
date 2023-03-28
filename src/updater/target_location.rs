use std::path::PathBuf;

pub fn install_location() -> PathBuf {
    let install_location = itex_app_data_folder();
    if !install_location.is_dir() {
        std::fs::create_dir(&install_location).expect("Could not create ITex folder");
    }

    install_location
}

pub fn itex_app_data_folder() -> PathBuf {
    if cfg!(windows) {
        let local_app_data =
            std::env::var("LOCALAPPDATA").expect("could not find local app data folder");

        let mut local_app_data = PathBuf::from(local_app_data);
        local_app_data.push("itex");
        local_app_data
    } else {
        // if UNIX
        let mut out_folder = PathBuf::from(std::env::var("HOME").unwrap());
        out_folder.push(".local");
        out_folder.push("share");
        out_folder.push("itex");

        out_folder
    }
}
