use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

mod input;
mod statistic;
use statistic::statistic as stats;
mod check;
use check::check as checker;

#[allow(unused_assignments)]
fn main() {
    let path = input::getpath();
    let ends = input::getends();
    let linenumbers = input::getlinenumbers();
    let stats = input::getstatistics();
    let lines = input::getlines();
    let quiet = input::getquiet();

    let errormsg: &str = "Could not write";
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    let mut total_lines: usize = 0;
    let lines_splitted: Vec<&str> = lines.split(":").collect();
    let mut output: String = String::from("");

    checker::check_warnings();
    checker::check_file(&path);

    let input_path = path;
    if stats {
        if input::getsize() {
            stats::getsize(&input_path);
        } else {
            stats::getdisksize(&input_path);
        }
    }
    let mut my_line;
    let file = BufReader::new(File::open(&input_path).unwrap());

    for line in file.lines() {
        my_line = line.unwrap();
        total_lines = total_lines + 1;

        if process_line(&lines_splitted, total_lines) {
            if stats {
                stats::addchars(&my_line);
            }

            if linenumbers {
                output.push_str(&(total_lines.to_string() + " │ "));
            }

            if ends {
                output.push_str(&(my_line + "$\n"));
            } else {
                output.push_str(&(my_line + "\n"));
            }

            if !quiet {
                if (total_lines % input::getbuffer()) == 0 {
                    write!(handle, "{}", &output).expect(errormsg);
                    output = String::from("");
                    my_line = String::from("");
                }
            }
        }
    }

    if output != "" && !quiet {
        write!(handle, "{}", &output).expect(errormsg);
        output = String::from("");
    }

    if stats {
        output = String::from("");
        if !quiet {
            output.push_str(&(stats::divider() + "\n"));
        }
        output.push_str(&("lines read: \t\t\t".to_owned() + &total_lines.to_string()));
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
        if lines_splitted[0].parse::<usize>().unwrap() <= total_lines
            && lines_splitted[1].parse::<usize>().unwrap() >= total_lines
        {
            return true;
        }
        return false;
    } else {
        return false;
    }
}
