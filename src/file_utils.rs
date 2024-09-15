use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

pub fn read_file(file_name: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut input = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if !line.starts_with('#') {
            input.push(line);
        }
    }

    Ok(input)
}

pub fn write_file(data: &[String], file_name: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)?;

    for line in data {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
pub fn comment_out_in_file(file_name: &str, line_to_comment: &str) -> Result<(), io::Error> {
    let file_path = Path::new(file_name);
    let mut lines = vec![];

    // Read all lines
    if let Ok(file) = File::open(file_path) {
        for line in BufReader::new(file).lines() {
            let line = line?;
            if line == line_to_comment {
                lines.push(format!("#{}", line));
            } else {
                lines.push(line);
            }
        }
    }

    // Write the modified lines back to the file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
