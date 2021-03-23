use std::fs;
use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=src/bin");
    let path = fs::read_dir("src/bin").unwrap();
    let mut file = fs::File::create("bins").unwrap();

    for bin in path {
        let dir_entry = bin.unwrap();
        let name = dir_entry.file_name();
        let name = name.to_str().unwrap();
        let len = name.len();
        file.write_fmt(format_args!("{}\n", &name[0..len - 3])).unwrap();
    }
}