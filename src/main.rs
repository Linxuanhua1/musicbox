
use musicbox::{utils, collect_tasks};
use std::io;


fn main() {
    loop {
        println!("请输入路径：");
        let mut path = String::new();
        io::stdin().read_line(&mut path).unwrap();

        let path = match utils::check_path(&path) {
            Ok(p) => p,
            Err(e) => {eprintln!("{}", e);
            continue
            }
        };
        let tasks = collect_tasks::collect_tasks(path);

        println!("{:?}", tasks);
    }
}

