use config_struct::{Error, StructOptions};
use dirs;

#[allow(unused_must_use)]
fn main() {
    let paths: [&str; 2] = ["./config.yaml", &(dirs::home_dir().unwrap().to_str().unwrap().to_owned()  + "/.config/dog/config.yaml")];

    for path in &paths {
        if std::path::Path::new(&path).exists() {
            create_config(&path);
        }
    }
}

#[allow(unused_must_use)]
fn create_config(path: &str) -> Result<(), Error> {
    println!("{}", path);
    config_struct::create_struct(
        path, 
        "src/input/config.rs",
        &StructOptions::serde_default()
    )
}