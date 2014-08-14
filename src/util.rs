use std::io::IoResult;
use std::io::fs::{readdir, copy, lstat, mkdir};
use std::collections::HashSet;

// copies directory `from` to directory `to`. all of the files under `from`
// will show up under `to`. Optionally creates `to` as well. Optionally takes
// a collection of Paths that should be ignored in the `from` directory
pub fn copy_dir_ignore(from: &Path, to: &Path, create: bool, ignore: &HashSet<Path>) -> IoResult<()> {
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
            let pnew = to.join( p.path_relative_from(from).unwrap() );

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


// reads a directory and of all the directories whose names are just numbers,
// return the biggest number. if no number directories, return None
pub fn get_highest_numdir(path: &Path) -> IoResult<Option<uint>> {
    let mut highest = None;

    for p in try!(readdir(path)).iter().filter(|x| x.is_dir()) {
        let pnew = p.path_relative_from(path).unwrap();
        let x: uint = match pnew.as_str() {
            None => continue,
            Some(s) =>
                match from_str(s) {
                    None => continue,
                    Some(s) => s
                }
        };

        if highest.is_none() || x > highest.unwrap() {
            highest = Some(x);
        }
    }

    Ok(highest)
}
