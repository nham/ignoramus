use std::io;
use std::io::IoResult;
use std::io::fs::{readdir, copy, lstat, mkdir};
use std::collections::HashSet;
use std::os;

// copies directory `from` to directory `to`. all of the files under `from`
// will show up under `to`. Optionally creates `to` as well. Optionally takes
// a collection of Paths that should be ignored in the `from` directory
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
    if igno_is_init() {
        Ok(false)
    } else {
        try!(mkdir(&Path::new(".igno"), io::UserDir));
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

fn get_next_snapshot_num() -> IoResult<uint> {
    match get_highest_numdir(&Path::new(".igno")) {
        Err(e) => Err(e),
        Ok(None) => Ok(0u),
        Ok(Some(n)) => Ok(n+1),
    }
}

fn igno_is_init() -> bool {
    Path::new(".igno").is_dir()
}


fn snapshot() -> IoResult<()> {
    let curr = Path::new(".");
    let ig_path = Path::new(".igno");

    let next_rev = match get_next_snapshot_num() {
        Err(e) => return Err(e),
        Ok(n) => n,
    };

    let mut ignore = HashSet::new();
    ignore.insert(ig_path.clone());

    let snapshot_path = ig_path.join(next_rev.to_string());
    copy_dir_ignore(&curr, &snapshot_path, true, &ignore)
}


enum Command {
    Init,
    Snapshot,
    Checkout(uint),
}

fn main() {
    let args = os::args();

    let mut cmd = None;
    if args.len() == 3 {
        if args[1].equiv(&"checkout") {
            if args[2].equiv(&"latest") {
                match get_highest_numdir(&Path::new(".igno")) {
                    Err(e) => fail!("{}", e),
                    Ok(None) => fail!("No snapshots have been made yet."),
                    Ok(Some(n)) => cmd = Some(Checkout(n)),
                }
            } else {
                let d: Option<uint> = from_str(args[2].as_slice());
                match d {
                    None => {},
                    Some(n) => cmd = Some(Checkout(n)),
                }
            }

            if cmd.is_none() {
                fail!("Argument to 'checkout' must either be an integer or 'latest'");
            }
        }
    } else if args.len() == 2 && args[1].equiv(&"init") {
        cmd = Some(Init);
    } else if args.len() == 1 {
        if !igno_is_init() {
            fail!("This is not an ignoramus repository");
        } else {
            cmd = Some(Snapshot);
        }
    }

    let cmd = match cmd {
        None => fail!("Command not recognized"),
        Some(c) => c,
    };

    match cmd {
        Init =>
            match igno_init() {
                Err(e) => println!("Error: {}", e),
                Ok(false) => println!("Repository already exists."),
                Ok(true) => println!("Initialized empty ignoramus repository"),
            },
        Snapshot =>
            match snapshot() {
                Err(e) => println!("Error: {}", e),
                Ok(_) => println!("Snapshot created"),
            },
        Checkout(n) => fail!("oops. unimplemented"),
    }
}
