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

// reads a directory and of all the directories whose names are just numbers,
// return the biggest number. if no number directories, return None
fn get_highest_numdir(path: &Path) -> IoResult<Option<uint>> {
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


fn main() {
    let curr = Path::new(".");
    let ig_path = Path::new(".igno");

    match igno_init() {
        Err(e) => println!("{}", e),
        Ok(false) => {},
        Ok(true) => println!("Created .igno"),
    }


    let next_rev = match get_highest_numdir(&ig_path) {
        Err(e) => { println!("{}", e); return; },
        Ok(None) => 0u,
        Ok(Some(n)) => n+1,
    };

    println!("{}", next_rev);

    let mut ignore = HashSet::new();
    ignore.insert(ig_path.clone());
    println!("{}", copy_dir_ignore(&curr, &ig_path.join(next_rev.to_string()), true, &ignore));
}
