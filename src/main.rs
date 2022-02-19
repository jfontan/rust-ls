use std::fs;
use std::io;
use std::io::prelude::*;
use std::os::unix::fs::MetadataExt;
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

const MODE_STRING: &[u8] = b"rwxrwxrwx";

fn format_mode(mode: u32) -> String {
    let mut str = String::from("");
    let mut mask = 0b100_000_000;

    println!("{}", mode);

    for i in 0..=8 {
        let r = mode & mask;
        if r > 0 {
            str.push(MODE_STRING[i] as char);
        } else {
            str.push('-');
        }
        mask = mask >> 1;
    }
    str
}

fn format_user(uid: u32) -> String {
    match users::get_user_by_uid(uid) {
        Some(user) => user.name().to_str().unwrap().to_string(),
        None => uid.to_string(),
    }
}

fn format_group(gid: u32) -> String {
    match users::get_group_by_gid(gid) {
        Some(group) => group.name().to_str().unwrap().to_string(),
        None => gid.to_string(),
    }
}

fn format_file(entry: &fs::DirEntry) -> io::Result<String> {
    let path = entry.path();
    let metadata = fs::symlink_metadata(path)?;

    let directory = if metadata.is_dir() { "d" } else { "-" };
    let mode = format_mode(metadata.mode());

    let links = metadata.nlink();

    let line = format!(
        "{}{} {} {} {} {}",
        directory,
        mode,
        links,
        format_user(metadata.uid()),
        format_group(metadata.gid()),
        entry.file_name().to_str().unwrap(),
    );
    Ok(line)
}

fn main() -> std::io::Result<()> {
    let path = create_test_files()?;

    let dir = fs::read_dir(&path)?;
    for entry in dir {
        let file = entry?;
        let line = format_file(&file)?;
        println!("{}", line);
    }

    delete_test_files(path)
}
