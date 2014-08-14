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

        match File::create(&head_path) {
            Err(e) => fail!("Couldn't create file: {}", e),
            Ok(_) => {},
        }

        Ok(true)
    }
}

fn igno_is_init() -> bool {
    Path::new(".igno").is_dir()
}

fn get_next_snapshot_num() -> IoResult<uint> {
    match get_highest_snapshot_num() {
        Err(e) => Err(e),
        Ok(None) => Ok(0u),
        Ok(Some(n)) => Ok(n+1),
    }
}

fn get_highest_snapshot_num() -> IoResult<Option<uint>> {
    get_highest_numdir(&Path::new(".igno"))
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
    CheckoutLatest,
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
        CheckoutLatest => {
            match get_highest_snapshot_num() {
                Err(e) => println!("Error: {}", e),
                Ok(None) => println!("There aren't any snapshots to check out."),
                Ok(Some(n)) => exec(Checkout(n)),
            }
        },
    }
}

fn parse_args(args: &[String]) -> Result<Command, &'static str> {
    let cnr = "Command not recognized.";
    let checkout_arg = "Argument to 'checkout' must either be an integer or 'latest'";
    match args.len() {
        2 =>
            if args[0].equiv(&"checkout") {
                if args[1].equiv(&"latest") {
                    Ok(CheckoutLatest)
                } else {
                    // try to parse argument as uint
                    let d: Option<uint> = from_str(args[1].as_slice());
                    match d {
                        None => Err(checkout_arg),
                        Some(n) => Ok(Checkout(n)),
                    }
                }
            } else {
                Err(cnr)
            },
        1 =>
            if args[0].equiv(&"init") {
                Ok(Init)
            } else {
                Err(cnr)
            },
        0 =>
            if igno_is_init() {
                Ok(Snapshot)
            } else {
                Err("This is not an ignoramus repository")
            },
        _ => Err(cnr),
    }
}

fn main() {
    let args = os::args();

    match parse_args(args.slice_from(1)) {
        Err(e) => fail!("{}", e),
        Ok(c) => exec(c),
    }
}
