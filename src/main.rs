use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::fs::{self, metadata};
use std::process;

mod input;
mod statistic;
use statistic::statistic as stats;

fn main() {
    let path = input::getpath();
    let ends = input::getends();
    let linenumbers = input::getlinenumbers();
    let stats = input::getstatistics();
    let lines = input::getlines();

    let errormsg: &str = "Could not write";
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    let mut total_lines: usize = 0;
    let lines_splitted: Vec<&str> = lines.split(":").collect();
    let mut output: String = String::from("");
    
    let metadata = metadata(&path);

    if !fs::metadata(&path).is_ok() {
        println!("dog: {}: No such file or directory", &path.display());
        process::exit(1);
    } else if metadata.unwrap().is_dir() {
        println!("dog: {}: Is a directory", path.display());
        process::exit(1);
    }

    let input_path = path;
    if stats {
        stats::getsize(&input_path);
    }

    let file = BufReader::new(File::open(&input_path).unwrap());
    for line in file.lines() {
        let my_line = line.unwrap();
        total_lines = total_lines + 1;

        if process_line(&lines_splitted, total_lines) {
            if stats {
                stats::addchars(&my_line);
            }

            if linenumbers {
               output.push_str(&(total_lines.to_string() + " │ "));
            }

            if ends {
                output.push_str(&(my_line + "$"));
            } else {
                output.push_str(&my_line);
            }
            writeln!(handle, "{}", &output).expect(errormsg);
            output = String::from("");
        }
    }

    if stats {
        output.push_str(&stats::divider());
        output.push_str(&("\nlines read: \t\t\t".to_owned() + &total_lines.to_string()));
        output.push_str(&stats::returnstats());

        writeln!(handle, "{}", &output).expect(errormsg);
    }
}

fn process_line(lines_splitted: &Vec<&str>, total_lines: usize) -> bool {
    if lines_splitted[0].is_empty() && lines_splitted[1].is_empty() {
        return true;
    } else if lines_splitted[0].is_empty() && !lines_splitted[1].is_empty() {
        if lines_splitted[1].parse::<usize>().unwrap() >= total_lines {
            return true;
        }
        return false;
    } else if !lines_splitted[0].is_empty() && lines_splitted[1].is_empty() {
        if lines_splitted[0].parse::<usize>().unwrap() <= total_lines {
            return true;
        }
        return false;
    } else if !lines_splitted[0].is_empty() && !lines_splitted[1].is_empty() {
        if lines_splitted[0].parse::<usize>().unwrap() <= total_lines && lines_splitted[1].parse::<usize>().unwrap() >= total_lines {
            return true;
        }
        return false;
    } else {
        return false;
    }
}