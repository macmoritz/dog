use structopt::StructOpt;

// mod config;

/// dog - a cat alternative written in rust
#[derive(StructOpt)]
pub struct Cli {
    /// Give me a file
    #[structopt(parse(from_os_str))]
    // ls: #[structopt(parse(from_os_str), default_value = ".")]
    path: std::path::PathBuf,

    /// Print $ at EOL
    #[structopt(short = "e", long = "show-ends")]
    show_ends: bool,

    /// Print statistics (count of lines, characters and uni-characters, actual size of file) at the end of the File
    #[structopt(short = "s", long = "statistics")]
    show_statistics: bool,

    /// Print line numbers
    #[structopt(short = "n", long = "line_numbers")]
    show_line_numbers: bool,

    /// Print a specified range of lines
    #[structopt(name = "n:m", short = "l", long = "lines", default_value = ":")]
    lines: String
}

// fn getConfig() {
//     if config::CONFIG {
//         let config = config::CONFIG;
//     } else {
//         let config = null;
//     }
// }

pub fn getpath() -> std::path::PathBuf {
    return Cli::from_args().path;
}

pub fn getends() -> bool {
    // if config::CONFIG.only_use_config {
    //     return config::CONFIG.features.show_ends;
    // } else {
    return Cli::from_args().show_ends;
    // }
}

pub fn getlinenumbers() -> bool {
    // if config::CONFIG.only_use_config {
    //     return config::CONFIG.features.line_numbers;
    // } else {
    return Cli::from_args().show_line_numbers;
    // }
}

pub fn getstatistics() -> bool {
    // if config::CONFIG.only_use_config {
    //      return config::CONFIG.features.show_statistics;
    // } else {
    return Cli::from_args().show_statistics;
    // }
}

pub fn getlines() -> String {
    return Cli::from_args().lines;
}
