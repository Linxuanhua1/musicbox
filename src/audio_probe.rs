use std::process::Command;
use std::fs;
use std::path;
use crate::consts;

pub fn probe(path: &path::Path) -> serde_json::Value {
    let file_path = path.to_str().unwrap();
    let args: Vec<&str> = vec!["-v", "quiet", "-print_format", "json", "-show_entries",
                               "stream=sample_fmt,codec_name,bits_per_raw_sample,bits_per_sample,channels,sample_rate,duration,bit_rate",
                               file_path];
    let output = Command::new("./bin/ffprobe").args(&args).output().unwrap();
    serde_json::from_str(&str::from_utf8(&output.stdout).unwrap()).unwrap()
}

pub fn is_allowed_to_converted(path: &path::Path) -> bool {
    let probe_data = &probe(path)["streams"][0];
    let codec_name = probe_data["codec_name"].as_str().unwrap();
    match codec_name {
        "aac" => {
            println!("{}是有损音频不会转换", path.display());
            return false;
        }
        "flac" => {
            let sample_rate: f64 = probe_data["sample_rate"].as_str().unwrap().parse().unwrap();
            let channels: f64 = probe_data["channels"].as_f64().unwrap();
            let bits_per_sample: f64 = probe_data["bits_per_raw_sample"].as_str().unwrap().parse().unwrap();
            let duration: f64 = probe_data["duration"].as_str().unwrap().parse().unwrap();
            let pcm_size: f64 = sample_rate * channels * bits_per_sample * duration / 8.0;
            let flac_size = fs::metadata(path).unwrap().len() as f64;
            return flac_size / pcm_size > 0.9;
        }
        _ => {}
    }
    let sample_fmt = probe_data["sample_fmt"].as_str().unwrap();
    if consts::ALLOWED_AUDIO_SAMPLE_FMT.contains(&sample_fmt){
        true
    } else {
        println!("音频文件{}不支持的编码格式，编码格式为{}", path.display(), sample_fmt);
        false
    }
}

