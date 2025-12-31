use std::path;
use crate::audio_encoder;

fn direct2flac(file_path: path::PathBuf) {
    let location = file_path.parent().unwrap();
    let file_name = file_path.file_name().unwrap();
    let output_file_name = format!("{}.flac", file_name.to_str().unwrap());
    let output_file_path = location.join(output_file_name);
    println!("正在将{}转化为flac8文件", file_path.display());
    audio_encoder::encode_file2flac(file_path, output_file_path);
    

}