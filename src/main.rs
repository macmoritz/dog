use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use structopt::StructOpt;

/// dog - a cat alternative written in rust
#[derive(StructOpt, Debug)]
pub struct Cli {
    /// Give me a file
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    /// Print $ at EOL
    #[structopt(short = "e", long = "show-ends")]
    show_ends: bool,

    /// Print a specified range of lines
    #[structopt(name = "n:m", short = "l", long = "lines", default_value = ":")]
    lines: String,

    /// Print statistics (count of lines, characters and uni-characters) at the end of the File
    #[structopt(short = "s", long = "statistics")]
    stats: bool,

    /// Print line numbers
    #[structopt(short = "n", long = "line_numbers")]
    line_numbers: bool
}

fn main() {
    let args = Cli::from_args();
    let directory = &args.path;
    let show_ends = &args.show_ends;
    let num_lines = &args.lines;
    let stats = &args.stats;
    let line_numbers = &args.line_numbers;

    let errormsg: &str = "Could not write";
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    let mut total_chars: usize = 0;
    let mut total_lines: usize = 0;
    let mut total_uni_chars: usize = 0;
    let num_lines_splitted: Vec<&str> = num_lines.split(":").collect();

    let input_path = directory;
    let file = BufReader::new(File::open(&input_path).unwrap());
    for line in file.lines() {
        let my_line = line.unwrap();
        total_lines = total_lines + 1;

        if process_line(&num_lines_splitted, total_lines) {
            if *stats {
                total_chars = total_chars + my_line.len();
                total_uni_chars = total_uni_chars + my_line.chars().count();
            }

            if *line_numbers {
                write!(handle, "{}", total_lines.to_string() + " | ").expect(errormsg);
            }

            if *show_ends {
                writeln!(handle, "{}", my_line + "$").expect(errormsg);  
            } else {
                writeln!(handle, "{}", &my_line).expect(errormsg);
            }
        }
    }

    if *stats {
        writeln!(handle, "test{}", total_lines).expect(errormsg);
        writeln!(handle, "Lines processed:\t\t{}", total_lines).expect(errormsg);
        writeln!(handle, "Characters read:\t\t{}", total_chars).expect(errormsg);
        writeln!(handle, "Unicode Characters read:\t{}", total_uni_chars).expect(errormsg);
    }
}

fn process_line(num_lines_splitted: &Vec<&str>, total_lines: usize) -> bool {
    if num_lines_splitted[0].is_empty() && num_lines_splitted[1].is_empty() {
        return true;
    } else if num_lines_splitted[0].is_empty() && !num_lines_splitted[1].is_empty() {
        if num_lines_splitted[1].parse::<usize>().unwrap() >= total_lines {
            return true;
        }
        return false;
    } else if !num_lines_splitted[0].is_empty() && num_lines_splitted[1].is_empty() {
        if num_lines_splitted[0].parse::<usize>().unwrap() <= total_lines {
            return true;
        }
        return false;
    } else if !num_lines_splitted[0].is_empty() && !num_lines_splitted[1].is_empty() {
        if num_lines_splitted[0].parse::<usize>().unwrap() <= total_lines && num_lines_splitted[1].parse::<usize>().unwrap() >= total_lines {
            return true;
        }
        return false;
    } else {
        return false;
    }
}