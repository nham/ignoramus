use std::io;
use std::io::IoResult;
use std::io::fs::{readdir, copy, lstat, mkdir};
use std::collections::HashSet;


fn copy_dir_ignore(from: &Path, to: &Path, create: bool, ignore: &HashSet<Path>) -> IoResult<()> {
    if !from.is_dir() {
        fail!("source isn't a directory");
    }

    if create {
        try!(mkdir(to, try!(lstat(from)).perm));
        println!("created {}", to.display());
    }

    for p in try!(readdir(from)).iter() {
        if ignore.contains(p) {
            continue;
        } else {
            let mut vec = Vec::from_slice(to.as_vec());

            if from != &Path::new(".") {
                let n = from.as_vec().len();
                vec.push_all( p.as_vec().slice_from(n) );
            } else {
                vec.push('/' as u8);
                vec.push_all( p.as_vec() );
            };

            let pnew = Path::new(vec);

            if p.is_dir() {
                try!(copy_dir_ignore(p, &pnew, true, ignore));
            } else {
                try!(copy(p, &pnew));
                println!("copied {} to {}", p.display(), pnew.display());
            }
        }
    }
    Ok(())
}

// If the .igno directory already exists, return false. If it was successfully
// created, return true. Otherwise an error is returned
fn igno_init() -> IoResult<bool> {
    let igno = Path::new(".igno");
    if igno.is_dir() {
        Ok(false)
    } else {
        try!(mkdir(&igno, io::UserDir));
        Ok(true)
    }
}


fn main() {
    let curr = Path::new(".");
    let ig_path = Path::new(".igno");

    match igno_init() {
        Err(e) => println!("{}", e),
        Ok(false) => {},
        Ok(true) => println!("Created .igno"),
    }

    let mut ignore = HashSet::new();
    ignore.insert(ig_path.clone());
    println!("{}", copy_dir_ignore(&curr, &ig_path, false, &ignore));
}
