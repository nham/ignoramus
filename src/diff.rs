use std::io::BufferedReader;
use std::io::File;
use std::path::BytesContainer;

use graph::{ses, Ins, Del};

mod graph;

fn file_to_vec(path: &Path) -> Vec<String> {
    let mut file = BufferedReader::new(File::open(path));
    let mut vec = vec!();
    for line in file.lines() {
        vec.push(line.unwrap());
    }
    vec
}

// I guess ideally this should not print but should return an iterator
// over strings
fn diff_files<T: BytesContainer>(file1: T, file2: T) {
    let path1 = Path::new(file1);
    let path2 = Path::new(file2);
    let vec1 = file_to_vec(&path1);
    let vec2 = file_to_vec(&path2);
    let ses = ses(vec1.as_slice(), vec2.as_slice());
    for &c in ses.iter() {
        match c {
            Ins(n) => print!("+{}", vec2[n]),
            Del(n) => print!("-{}", vec1[n]),
        }
    }
}

fn main() {
    let a = vec!('a', 'b', 'd', 'c');
    let b = vec!('a', 'b', 'f');

    println!("{}", ses(a.as_slice(), b.as_slice()));
    println!("{}", ses(&[0u, 1, 2, 3], &[0, 1, 2, 3]));


    println!("------------");
    diff_files("docs/diff.md", "docs/diff2.md");
}
