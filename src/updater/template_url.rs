use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug)]
struct VersionData {
    // assets: Vec<String>,
    name: String,
}

pub fn get_template_url() -> Url {
    let client = reqwest::blocking::Client::new();
    let version_json = client
        .get("https://api.github.com/repos/oneelectron/itex/releases/latest")
        .header("User-Agent", "reqwest");

    let version_json = version_json.send().expect("Could not connect to GitHub").text().unwrap();

    let version_data: VersionData = serde_json::from_str(version_json.as_str()).unwrap();

    println!("{}", version_data.name);

    // https://github.com/oneElectron/itex/releases/download/v1.0.1/itex-templates.zip
    let mut base_url = "https://github.com/oneElectron/itex/releases/download/".to_string();
    base_url.push_str(version_data.name.as_str());
    base_url.push_str("/itex-templates.zip");
    let outurl = Url::parse(base_url.as_str()).unwrap();

    outurl
}
