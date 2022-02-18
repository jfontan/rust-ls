use std::fs;
use std::io;
use std::io::prelude::*;
use tempfile::tempdir;

fn create_test_files() -> io::Result<String> {
    let files = vec!["zero", "one", "two", "three", ".four"];

    let dir = tempdir()?;
    let path = dir.into_path().to_str().unwrap().to_owned();

    for (size, name) in files.iter().enumerate() {
        let p = format!("{}/{}", path, name);
        println!("{}", p);

        let mut f = fs::File::create(p)?;
        for _ in 0..size {
            f.write(b"x")?;
        }
    }

    Ok(path)
}

fn delete_test_files(path: String) -> io::Result<()> {
    fs::remove_dir_all(path)?;
    Ok(())
}

const mode_string: &[u8] = b"rwxrwxrwx";

fn format_mode(mode: u32) -> String {
    let mut str = String::from("");
    let mut mask = 0b100_000_000;

    for i in 8..=0 {
        let r = mode & mask;
        if r > 0 {
            str.push(mode_string[i] as char);
        } else {
            str.push('-');
        }

    }
    str
}

fn format_file(path: String) -> io::Result<String> {
    let metadata: fs::MetadataEx = fs::symlink_metadata(path)?;
    mode = format_mode(metadata.
    Ok("".to_string())
}

fn main() -> std::io::Result<()> {
    let path = create_test_files()?;

    let dir = fs::read_dir(&path)?;
    for entry in dir {
        let file = entry?;
        println!("{:?} {:?}", file.file_name(), file.file_type());
        format_file(file.path().to_str().unwrap().to_owned())?;
    }

    delete_test_files(path)
}
