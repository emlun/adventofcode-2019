use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

pub type Solution = (String, String);

pub fn day_input_filename(day: u8) -> PathBuf {
    let padded_day = format!("{:02}", day);
    Path::new("inputs").join(format!("day{}.in", padded_day))
}

pub fn get_file_lines(path: &Path) -> Result<Vec<String>, std::io::Error> {
    if path == Path::new("-") {
        read_lines(std::io::stdin())
    } else {
        read_lines(
            File::open(&path)
                .unwrap_or_else(|_| panic!(format!("Input file not found: {:?}", path))),
        )
    }
}

fn read_lines<I: Read>(mut source: I) -> Result<Vec<String>, std::io::Error> {
    let mut contents: String = String::new();
    source.read_to_string(&mut contents)?;
    Ok(contents.lines().map(&str::to_string).collect())
}
