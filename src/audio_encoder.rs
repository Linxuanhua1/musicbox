use std::io::Write;
use std::path;
use std::process::{Command, Stdio};


/// 用于将wav文件转换为flac
pub fn encode_file2flac(
    input: impl AsRef<path::Path>,
    output: impl AsRef<path::Path>,
) -> Result<(), std::io::Error> {
    let status = Command::new("./bin/flac")
        .arg(input.as_ref())
        .arg("--best")
        .arg("--thread=16")
        .arg("-o")
        .arg(output.as_ref())
        .status()?;

    if !status.success() {
        eprintln!(
            "wav → flac 失败\n输入：{}\n输出：{}",
            input.as_ref().display(),
            output.as_ref().display()
        );
    }

    Ok(())
}


/// 用于将整轨文件的分轨转换为flac
pub fn encode_pcm2flac(
    pcm_data: &[u8],
    output: impl AsRef<path::Path>,
    sample_rate: u32,
    channels: u16,
    bps: u16,
) -> Result<(), std::io::Error> {
    let args = vec![
        "--force-raw-format".into(),
        "--sign=signed".into(),
        "--endian=little".into(),
        format!("--channels={channels}"),
        format!("--sample-rate={sample_rate}"),
        format!("--bps={bps}"),
        "-".into(),
        "--best".into(),
        "--thread=16".into(),
        "-o".into(),
        output.as_ref().to_string_lossy().into_owned(),
    ];

    run_flac_with_stdin(&args, pcm_data)?;

    println!(
        "已将 PCM 数据编码为 flac：{}",
        output.as_ref().display()
    );

    Ok(())
}


/// 用于将别的格式生成的wav数据转换为flac
pub fn encode_wav_bytes2flac(
    wav_bytes: &[u8],
    output: impl AsRef<path::Path>,
) -> Result<(), std::io::Error> {
    let args = vec![
        "-".into(),
        "--best".into(),
        "--thread=16".into(),
        "-o".into(),
        output.as_ref().to_string_lossy().into_owned(),
    ];

    run_flac_with_stdin(&args, wav_bytes)?;

    println!(
        "已将 wav bytes 转换为 flac：{}",
        output.as_ref().display()
    );

    Ok(())
}


fn run_flac_with_stdin(
    args: &[String],
    input: &[u8],
) -> Result<(), std::io::Error> {
    let mut child = Command::new("./bin/flac")
        .args(args)
        .stdin(Stdio::piped())
        .spawn()?;

    // 写入 stdin
    {
        let stdin = child.stdin.as_mut().expect("stdin not available");
        stdin.write_all(input)?;
    }
    // 作用域结束，stdin 自动 drop，发送 EOF

    let status = child.wait()?;

    if !status.success() {
        eprintln!("flac 执行失败，exit code = {:?}", status.code());
    }

    Ok(())
}
