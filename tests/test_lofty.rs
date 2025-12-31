
#[test]
#[ignore]
fn read_tags() {
    use lofty::read_from_path;
    use lofty::file::TaggedFileExt;

    let tagged_file = read_from_path(r"path").unwrap();

    let tags = tagged_file.primary_tag().unwrap();

    for item in tags.items() {
        println!("key:{:?}, value:{:?}", item.key(), item.value());
    }
}