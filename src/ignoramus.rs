#![feature(default_type_params)]

use graph::ses;

mod graph;

fn main() {
    let a = vec!('a', 'b', 'd', 'c');
    let b = vec!('a', 'b', 'f');

    println!("{}", ses(a.as_slice(), b.as_slice()));
    println!("{}", ses(&[0u, 1, 2, 3], &[0, 1, 2, 3]));

}
