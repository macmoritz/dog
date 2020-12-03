use config_struct::{Error, StructOptions};

#[allow(unused_must_use)]
fn main() {
    // let paths: [&str; 3] = ["~/.config/dog/config.yaml", "./../config.yaml", "./config.yaml"];

    // for path in &paths {
        // if std::path::Path::new(&path).exists() {
            // create_config(&path);
            create_config("~/.config/dog/config.yaml");
        // }
    // }
}

#[allow(unused_must_use)]
fn create_config(path: &str) -> Result<(), Error> {
    config_struct::create_struct(
        path, 
        "src/input/config.rs",
        &StructOptions::serde_default()
    )
}