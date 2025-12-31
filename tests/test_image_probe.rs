
#[test]
#[ignore]
fn test_probe() -> () {
    use std::path::Path;
    use musicbox::image_probe::probe;
    let file_path = Path::new(r"C:\Users\Linxuanhua\Desktop\111\sample.jpg");
    let result = probe(&file_path);
    println!("{}", result[0]["image"]["colorspace"]);
}