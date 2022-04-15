use std::{
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let mut args = std::env::args();
    walkdir(args.nth(1).unwrap(), args.next().unwrap());
}

fn walkdir(path: impl AsRef<Path>, ext: impl AsRef<OsStr>) {
    let ext = ext.as_ref();
    for entry in path.as_ref().read_dir().unwrap() {
        let entry = entry.unwrap();
        let ty = entry.file_type().unwrap();
        let path = entry.path();
        if ty.is_file() && path.extension() == Some(ext) {
            let reader = BufReader::new(File::open(&path).unwrap());
            println!("{} : {}", path.display(), reader.lines().count());
        }
        if ty.is_dir() {
            walkdir(path, ext);
        }
    }
}
