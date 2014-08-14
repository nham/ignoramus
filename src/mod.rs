use util::{copy_dir_ignore, get_highest_numdir};

use std::io;
use std::io::IoResult;
use std::io::fs::{mkdir, File};
use std::collections::HashSet;
use std::os;

mod util;

// If the .igno directory already exists, return false. If it was successfully
// created, return true. Otherwise an error is returned
fn igno_init() -> IoResult<bool> {
    if igno_is_init() {
        Ok(false)
    } else {
        let ig_path = Path::new(".igno");
        try!(mkdir(&ig_path, io::UserDir));
        let head_path = ig_path.join("head");

        let mut file = match File::create(&head_path) {
            Err(e) => fail!("Couldn't create file: {}", e),
            Ok(file) => file,
        };

        try!(file.write_str("0\n"));
        Ok(true)
    }
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

fn update_head(n: uint) -> IoResult<()> {
    let head_path = Path::new(".igno").join("head");
    let mut file = match File::open_mode(&head_path, io::Open, io::Write) {
        Err(e) => return Err(e),
        Ok(file) => file,
    };

    file.write_str( (n.to_string() + "\n").as_slice() )
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
    try!(copy_dir_ignore(&curr, &snapshot_path, true, &ignore));
    update_head(next_rev)
}


fn checkout(n: uint) -> IoResult<()> {
    let snap_path = Path::new(".igno").join(n.to_string());
    let curr = Path::new(".");
    match copy_dir_ignore(&snap_path, &curr, false, &HashSet::new()) {
        Err(e) => Err(e),
        Ok(_) => update_head(n),
    }
}


enum Command {
    Init,
    Snapshot,
    Checkout(uint),
}

fn exec(cmd: Command) {
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
        Checkout(n) => {
            match checkout(n) {
                Err(e) => println!("Error: {}", e),
                Ok(_) => println!("Snapshot checked out"),

            }
        },
    }
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

    exec(cmd);
}
