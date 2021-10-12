use std::{env, fs, io};
use std::fs::DirEntry;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    visit_dir(path.as_path());
    visit_dirs(path.as_path(), cd);
    Ok(())
}

fn visit_dir(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            println!("{}", path.display())
        }
    }
    Ok(())
}

fn cd(d: &DirEntry) {
    println!("{}", d.file_name().to_str().unwrap())
}

fn visit_dirs(dir: &Path, cb: fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                println!("{:?}", path.file_name());
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
