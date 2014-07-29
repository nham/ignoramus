use std::io::IoResult;
use std::io::fs::{readdir, copy, stat, mkdir};
use std::collections::HashSet;

// creates a new path that is a concatenation of path1 + path2[n:], where
// path2[n:] denotes the bytes of path2 starting at the nth byte (zero-indexed)
// (essentially, it "replaces" the first n bytes of path2 with path1)
fn replace_front(path1: &Path, path2: &Path, n: uint) -> Path {
    let mut vec = Vec::from_slice(path1.as_vec());
    vec.push_all( path2.as_vec().slice_from(n) );
    Path::new(vec)
}


// TODO: probably doesnt correctly handle symlinks?
fn copy_dir_ignore(from: &Path, to: &Path, ignore: &HashSet<Path>) -> IoResult<()> {
    if !from.is_dir() {
        fail!("source isn't a directory");
    }

    try!(mkdir(to, try!(stat(from)).perm));
    println!("created {}", to.display());

    for p in try!(readdir(from)).iter() {
        if ignore.contains(p) {
            continue;
        } else {
            let pnew = replace_front(to, p, from.as_vec().len());
            if p.is_dir() {
                try!(copy_dir_ignore(p, &pnew, ignore));
            } else {
                try!(copy(p, &pnew));
                println!("copied {} to {}", p.display(), pnew.display());
            }
        }
    }
    Ok(())
}


fn main() {
    //let path = Path::new(".");
    let mut ignore = HashSet::new();
    ignore.insert(Path::new(".igno"));
    ignore.insert(Path::new(".git"));
    println!("{}", copy_dir_ignore(&Path::new("foo"), &Path::new("bar"), &ignore));
}
