#[allow(non_upper_case_globals)]
pub mod statistic {
    use std::fs;

    static mut total_chars: usize = 0;
    static mut total_uni_chars: usize = 0;
    static mut filesize: usize = 0;

    pub fn addchars(line: &String) {
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

    pub fn returnstats() -> String {
        let mut output: String = String::from("----\n");

        unsafe {
            output.push_str(format!("Characters read:\t\t{}\n", total_chars).as_str());
            output.push_str(format!("Unicode Characters read:\t{}\n", total_uni_chars).as_str());
            output.push_str(format!("Size of file:\t\t\t{} Bytes", filesize).as_str());
        }
        return output;
    }
}
