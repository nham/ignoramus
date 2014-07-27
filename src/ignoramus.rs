use std::io::BufferedReader;
use std::io::File;

use graph::ses;

mod graph;

fn file_to_vec(path: &Path) -> Vec<String> {
    let mut file = BufferedReader::new(File::open(path));
    let mut vec = vec!();
    for line in file.lines() {
        vec.push(line.unwrap());
    }
    vec
}

fn main() {
    let a = vec!('a', 'b', 'd', 'c');
    let b = vec!('a', 'b', 'f');

    println!("{}", ses(a.as_slice(), b.as_slice()));
    println!("{}", ses(&[0u, 1, 2, 3], &[0, 1, 2, 3]));


    let path = Path::new("docs/diff.md");
    let vec = file_to_vec(&path);
    let path = Path::new("docs/diff2.md");
    let vec2 = file_to_vec(&path);
    println!("{}", ses(vec.as_slice(), vec2.as_slice()));

}
