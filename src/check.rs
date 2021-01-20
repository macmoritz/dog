pub mod check {
    use std::fs::{self, metadata};
    use std::process;
    use colored::*;

    use crate::input;

    pub fn check_warnings() {
        if input::getouc() && input::getoucwarning() {
            println!("{}", "ONLY USING CONFIG\n".color("yellow").bold().underline());
        } 
    }

    pub fn check_file(path: &std::path::PathBuf) {
        let metadata = metadata(&path);

        if !fs::metadata(&path).is_ok() {
            println!("{}", format!("dog: {}: No such file or directory", &path.display()).color("red").bold());
            process::exit(1);
        } else if metadata.unwrap().is_dir() {
            println!("{}", format!("dog: {}: Is a directory", path.display()).color("red").bold());

            if input::getlistdir() {
                println!("{}", "Do you mean one of these files?".color("green"));
                list_dir(path, input::getlistdirdepth(), 1);
            }
            process::exit(1);
        }
    }

    fn list_dir(path: &std::path::PathBuf, depth: i64, indentation: usize) {
        let paths = fs::read_dir(path).unwrap();
        let mut local_depth = depth;

        for path in paths {
            let child = path.unwrap().path();
            let child_metadata = metadata(&child);
            let child_path: String = format!("{}", child.display());

            if child_metadata.unwrap().is_dir() {
                println!("{}{}", "\t".repeat(indentation), child_path.truecolor(122, 166, 218));
                if local_depth > 0 {
                    local_depth = local_depth - 1;
                    list_dir(&child, local_depth, indentation + 1);
                }
            } else {
                println!("{}{}", "\t".repeat(indentation), child_path.truecolor(185, 202, 74));
            }
        }
    }
}