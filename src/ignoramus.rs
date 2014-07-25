#![feature(default_type_params)]

use graph::ses;

mod graph;

fn main() {
    let a = vec!('a');
    let b = vec!('a', 'b');

    println!("{}", ses(a.as_slice(), b.as_slice()));

}
