use std::io::fs::{readdir, copy};
use std::collections::HashSet;

fn list_dir_rec(path: &Path, ignore: &HashSet<Path>) {
    let contents = 
        match readdir(path) {
            Err(_) => fail!("Couldn't do a thing"),
            Ok(v) => v,
        };

    for p in contents.iter() {
        if ignore.contains(p) {
            continue;
        } else if p.is_dir() {
            list_dir_rec(p, ignore);
        } else {
            println!("{}", p.display());
        }
    }
}


fn main() {
    let path = Path::new(".");
    let mut ignore = HashSet::new();
    ignore.insert(Path::new(".git"));
    ignore.insert(Path::new("target"));
    list_dir_rec(&path, &ignore);
}
