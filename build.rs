use std::io::Write;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    {
        let mut f = fs::File::create("src/snapshots/file.txt").unwrap();
        f.write_all(b"hello").unwrap();
    }

    sleep(Duration::from_millis(1000));
}
