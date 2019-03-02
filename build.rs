use std::io::Write;
use std::fs;

fn main() {
    let mut f = fs::File::create("ignored_folder/file.txt").unwrap();
    f.write_all(b"hello").unwrap();
}
