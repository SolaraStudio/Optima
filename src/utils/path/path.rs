use std::path::{Path, PathBuf};

pub fn join_paths(base: &str, child: &str) -> String {
    let path = Path::new(base).join(child);
    path.to_str().unwrap_or("").to_string()
}

pub fn normalize_path(path: &str) -> String {
    let path = Path::new(path);
    path.components()
        .filter(|c| !matches!(c, std::path::Component::CurDir))
        .collect::<PathBuf>()
        .to_str()
        .unwrap_or(path.to_str().unwrap_or(""))
        .to_string()
}

pub fn is_absolute(path: &str) -> bool {
    Path::new(path).is_absolute()
}

pub fn is_relative(path: &str) -> bool {
    !is_absolute(path)
}

pub fn parent_path(path: &str) -> Option<String> {
    Path::new(path).parent().and_then(|p| p.to_str()).map(|s| s.to_string())
}

pub fn file_name(path: &str) -> Option<String> {
    Path::new(path).file_name().and_then(|p| p.to_str()).map(|s| s.to_string())
}

pub fn file_stem(path: &str) -> Option<String> {
    Path::new(path).file_stem().and_then(|p| p.to_str()).map(|s| s.to_string())
}

pub fn extension(path: &str) -> Option<String> {
    Path::new(path).extension().and_then(|p| p.to_str()).map(|s| s.to_string())
}

pub fn has_extension(path: &str, ext: &str) -> bool {
    Path::new(path).extension().and_then(|p| p.to_str()) == Some(ext)
}

pub fn with_extension(path: &str, new_ext: &str) -> String {
    let mut path = PathBuf::from(path);
    path.set_extension(new_ext);
    path.to_str().unwrap_or(path).to_string()
}

pub fn without_extension(path: &str) -> String {
    let path = Path::new(path);
    if let Some(stem) = path.file_stem() {
        if let Some(parent) = path.parent() {
            return parent.join(stem).to_str().unwrap_or(path).to_string();
        }
        return stem.to_str().unwrap_or(path).to_string();
    }
    path.to_str().unwrap_or("").to_string()
}
