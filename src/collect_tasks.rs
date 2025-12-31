use walkdir::WalkDir;
use crate::{consts, audio_probe};
use std::path;

pub fn collect_tasks(path: path::PathBuf) -> Vec<String> {
    let mut tasks: Vec<String> = Vec::new();

    for entry in WalkDir::new(path).into_iter()
    {
        let entry = entry.unwrap();
        let file_path = entry.path();
        let ext = match file_path.extension() {
            Some(ext) => ext.to_str().unwrap(),
            None => continue
        };
        if consts::ALLOWED_AUDIO_FMT.contains(&ext) {
            if audio_probe::is_allowed_to_converted(file_path){
                tasks.push(file_path.display().to_string());
            }
        }
    }
    tasks
}