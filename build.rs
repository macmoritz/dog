use config_struct::{Error, StructOptions};
use dirs;

#[allow(unused_must_use)]
fn main() {
    let mut created_config: bool = false;
    let paths: [&str; 2] = [&(dirs::home_dir().unwrap().to_str().unwrap().to_owned()  + "/.config/elk/config.yaml"), "./config.yaml"];

    for path in &paths {
        if std::path::Path::new(&path).exists() && !created_config {
            if let Err(e) = create_config(&path) {
                eprintln!("Application error: {}", e);
            } else {
                created_config = true;
            }
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
