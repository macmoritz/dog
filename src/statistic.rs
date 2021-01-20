#[allow(non_upper_case_globals)]
pub mod statistic {
    use std::fs;
    use filesize::PathExt;

    static mut total_chars: usize = 0;
    static mut total_uni_chars: usize = 0;
    static mut filesize: usize = 0;

    pub fn addchars(line: &str) {
        unsafe {
            total_chars = total_chars + line.len();
            total_uni_chars = total_uni_chars + line.chars().count();
        }
    }

    pub fn getsize(path: &std::path::PathBuf) {
        unsafe {
            filesize = fs::metadata(path).unwrap().len() as usize;
        }
    }

    pub fn getdisksize(path: &std::path::PathBuf) {
        unsafe {
            filesize = path.size_on_disk().unwrap() as usize;
        }
    }

    pub fn returnstats() -> String {
        let mut output: String = String::from("");

        unsafe {
            output.push_str(format!("\nCharacters read:\t\t{}\n", total_chars).as_str());
            output.push_str(format!("Unicode Characters read:\t{}\n", total_uni_chars).as_str());
            output.push_str(format!("Real size of file:\t\t{}", formatsize(filesize)).as_str());
        }
        return output;
    }

    pub fn divider() -> String {
        use terminal_size::{Width, Height, terminal_size};
        
        let mut output: String = String::from("");

        let size = terminal_size();
        if let Some((Width(w), Height(_h))) = size {
            for _i in 0..w {
                output.push_str("—");
            }
        } else {
            output.push_str("————————");
        }
        
        return output;
    }

    fn formatsize(mut size: usize) -> String {
        if size > 1000000000000 {
            size = size / 1000000000000;
            return format!("{} {}", size, "TeraBytes");
        }

        if size > 1000000000 {
            size = size / 1000000000;
            return format!("{} {}", size, "GigaBytes");
        }

        if size > 1000000 {
            size = size / 1000000;
            return format!("{} {}", size, "MegaBytes");
        }

        if size > 1000 {
            size = size / 1000;
            return format!("{} {}", size, "KiloBytes");
        }
        return format!("{} {}", size, "Bytes");
    } 
}
