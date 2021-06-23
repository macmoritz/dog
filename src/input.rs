use structopt::StructOpt;
use std::convert::TryInto;

mod config;
use config::CONFIG;

/// dog - a cat alternative written in rust
#[derive(StructOpt)]
pub struct Cli {
    /// Give me a file
    #[structopt(parse(from_os_str))]
    // #[structopt(parse(from_os_str), default_value = ".")] // dog without path -> ls
    path: std::path::PathBuf,

    /// Do not print the file contents
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,

    /// Print $ at EOL
    #[structopt(short = "e", long = "show-ends")]
    show_ends: bool,

    /// Print statistics (count of lines, characters and uni-characters, actual size of file) at the end of the File
    #[structopt(short = "s", long = "statistics")]
    show_statistics: bool,

    /// print apparent sizes, rather than disk usage
    #[structopt(long = "size")]
    size: bool,

    /// Print line numbers
    #[structopt(short = "n", long = "linenumbers")]
    line_numbers: bool,

    /// Print a specified range of lines
    #[structopt(name = "n:m", short = "l", long = "lines", default_value = ":")]
    lines: String,
}

pub fn getouc() -> bool {
    return CONFIG.only_use_config.mode;
}

pub fn getoucwarning() -> bool {
    return CONFIG.only_use_config.warning;
}

pub fn getpath() -> std::path::PathBuf {
    return Cli::from_args().path;
}

pub fn getquiet() -> bool {
    return Cli::from_args().quiet;
}

pub fn getends() -> bool {
    if CONFIG.only_use_config.mode {
        return CONFIG.features.show_ends;
    } else {
        return Cli::from_args().show_ends;
    }
}

pub fn getlinenumbers() -> bool {
    if CONFIG.only_use_config.mode {
        return CONFIG.features.line_numbers;
    } else {
        return Cli::from_args().line_numbers;
    }
}

pub fn getstatistics() -> bool {
    if CONFIG.only_use_config.mode {
        return CONFIG.features.show_statistics;
    } else {
        return Cli::from_args().show_statistics;
    }
}

pub fn getsize() -> bool {
    if CONFIG.only_use_config.mode {
        return CONFIG.features.show_statistics;
    } else {
        return Cli::from_args().size;
    }
}

pub fn getlines() -> String {
    return Cli::from_args().lines;
}

pub fn getlistdir() -> bool {
    return CONFIG.default.list_dir;
}

pub fn getlistdirdepth() -> i64 {
    return CONFIG.default.list_dir_depth;
}

pub fn getbuffer() -> usize {
    return CONFIG.default.buffer.try_into().unwrap();
}
