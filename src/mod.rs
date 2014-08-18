use util::{copy_dir_ignore, get_highest_numdir};

use std::io;
use std::io::IoResult;
use std::io::fs::{mkdir, File};
use std::collections::HashSet;
use std::os;

mod util;

// If the .igno directory already exists, return false. If it was successfully
// created, return true. Otherwise an error is returned
fn igno_init() -> IoResult<()> {
    let ig_path = Path::new(".igno");
    try!(mkdir(&ig_path, io::UserDir));
    let head_path = ig_path.join("head");

    match File::create(&head_path) {
        Err(e) => Err(e),
        Ok(_) => Ok(()),
    }
}

fn igno_is_init() -> bool {
    Path::new(".igno").is_dir()
}

fn get_current() -> IoResult<uint> {
    let head_path = Path::new(".igno").join("head");
    let mut file = match File::open(&head_path) {
        Err(e) => return Err(e),
        Ok(file) => file,
    };

    match file.read_to_string() {
        Err(e) => Err(e),
        Ok(s) => Ok(from_str::<uint>(s.as_slice().slice_to(s.len() - 1)).unwrap()),
    }
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


fn commit(message: String) -> IoResult<()> {
    let curr = Path::new(".");
    let ig_path = Path::new(".igno");

    let next_rev = match get_next_snapshot_num() {
        Err(e) => return Err(e),
        Ok(n) => n,
    };

    let mut ignore = HashSet::new();
    ignore.insert(ig_path.clone());

    let snapshot_path = ig_path.join(next_rev.to_string());
    let tree_path = snapshot_path.join("tree");
    try!(mkdir(&snapshot_path, io::UserDir));
    try!(copy_dir_ignore(&curr, &tree_path, true, &ignore));

    // Write snapshot metadata (just commit message at the moment)
    match File::create(&snapshot_path.join("meta")) {
        Err(e) => fail!("Couldn't create meta file: {}", e),
        Ok(mut file) => try!(file.write_str( (message + "\n").as_slice() )),
    }

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
    Current,
    Commit(String),
    Checkout(uint),
    CheckoutLatest,
}

fn exec(cmd: Command) {
    // the only command we can execute when it's not already a repo is init
    match cmd {
        Init =>
            if igno_is_init() {
                println!("Current directory is already an ignoramus repository");
                return
            },
        _ =>
            if !igno_is_init() {
                println!("Current directory is not an ignoramus repository");
                return
            },
    }

    match cmd {
        Init =>
            match igno_init() {
                Err(e) => println!("Error: {}", e),
                Ok(_) => println!("Initialized empty ignoramus repository"),
            },
        Current =>
            match get_current() {
                Err(e) => println!("Error: {}", e),
                Ok(n) => println!("Current revision: {}", n),
            },
        Commit(s) =>
            match commit(s) {
                Err(e) => println!("Error: {}", e),
                Ok(_) => println!("Snapshot created"),
            },
        Checkout(n) =>
            match checkout(n) {
                Err(e) => println!("Error: {}", e),
                Ok(_) => println!("Snapshot checked out"),
            },
        CheckoutLatest =>
            match get_highest_snapshot_num() {
                Err(e) => println!("Error: {}", e),
                Ok(None) => println!("There aren't any snapshots to check out."),
                Ok(Some(n)) => exec(Checkout(n)),
            },
    }
}

fn parse_args(args: &[String]) -> Result<Command, &'static str> {
    let checkout_arg = "Argument to 'checkout' must either be an integer or 'latest'";
    let wna = "Wrong number of arguments.";

    if args.len() == 0 {
        return Err("No command given.");
    }

    match (args[0].as_slice(), args.len()) {
        ("checkout", 2) => {
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
        },
        ("checkout", _) => Err(wna),

        ("commit", 2) => Ok(Commit(args[1].to_string())),
        ("commit", _) => Err(wna),

        ("init", 1) => Ok(Init),
        ("init", _) => Err(wna),

        ("current", 1) => Ok(Current),
        ("current", _) => Err(wna),
        _ => Err("Command not recognized."),
    }
}

fn main() {
    let args = os::args();

    match parse_args(args.slice_from(1)) {
        Err(e) => println!("{}", e),
        Ok(c) => exec(c),
    }
}
