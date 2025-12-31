use std::process::Command;
use std::path;
use crate::consts;

pub fn probe(path: &path::Path) -> serde_json::Value {
    let file_path = path.to_str().unwrap();
    let args: Vec<&str> = vec![file_path, "json:-"];
    let output = Command::new("./bin/magick").args(&args).output().unwrap();
    serde_json::from_str(&str::from_utf8(&output.stdout).unwrap()).unwrap()
}

pub fn is_allowed_to_convert(path: &path::Path) -> bool {
    let color_space = &probe(path)[0];
    let color_space = &color_space["image"]["colorspace"].as_str().unwrap();
    if consts::ALLOWED_IMAGE_COLOR_MODEL.contains(color_space) {
        true
    } else {
        println!("图片文件{}为不支持的图片颜色空间{}", path.display(), color_space);
        false
    }
}




