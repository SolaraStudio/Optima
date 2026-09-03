use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn read_file_bytes(path: &str) -> Result<Vec<u8>, std::io::Error> {
    fs::read(path)
}

pub fn write_file(path: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn write_file_bytes(path: &str, data: &[u8]) -> Result<(), std::io::Error> {
    fs::write(path, data)
}

pub fn append_file(path: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn read_lines(path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

pub fn write_lines(path: &str, lines: &[String]) -> Result<(), std::io::Error> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for line in lines {
        writeln!(writer, "{}", line)?;
    }
    Ok(())
}

pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}

pub fn is_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn create_dir(path: &str) -> Result<(), std::io::Error> {
    fs::create_dir_all(path)
}

pub fn remove_file(path: &str) -> Result<(), std::io::Error> {
    fs::remove_file(path)
}

pub fn remove_dir(path: &str) -> Result<(), std::io::Error> {
    fs::remove_dir_all(path)
}

pub fn list_files(path: &str) -> Result<Vec<String>, std::io::Error> {
    let entries = fs::read_dir(path)?;
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name().to_str().unwrap_or("").to_string();
        if entry.file_type()?.is_file() {
            files.push(file_name);
        }
    }
    Ok(files)
}

pub fn list_dirs(path: &str) -> Result<Vec<String>, std::io::Error> {
    let entries = fs::read_dir(path)?;
    let mut dirs = Vec::new();
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name().to_str().unwrap_or("").to_string();
        if entry.file_type()?.is_dir() {
            dirs.push(file_name);
        }
    }
    Ok(dirs)
}
