use std::fmt;
use std::path::{Path, PathBuf};

pub fn check_path(input: &str) -> Result<PathBuf, PathError> {
    let input = input.trim();
    if input.trim().is_empty() {
        return Err(PathError::Empty);
    }

    let path = Path::new(input);

    if !path.exists() {
        return Err(PathError::NotExists);
    }

    if !path.is_dir() {
        return Err(PathError::NotDir);
    }

    Ok(path.to_path_buf())
}


pub enum PathError {
    Empty,
    NotExists,
    NotDir,
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathError::Empty => write!(f, "路径不能为空"),
            PathError::NotExists => write!(f, "路径不存在"),
            PathError::NotDir => write!(f, "路径不是一个目录"),
        }
    }
}