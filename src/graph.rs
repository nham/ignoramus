pub use std::collections::{RingBuf, HashMap, HashSet, Deque};
use std::iter;

use self::tree::Tree;

type NodeIndex = uint;
type NodeIndexSet = HashSet<NodeIndex>;

// unweighted directed graph
struct Digraph {
    nodes: HashSet<NodeIndex>,
    in_adj: HashMap<NodeIndex, NodeIndexSet>,
    out_adj: HashMap<NodeIndex, NodeIndexSet>,
}

struct NodeIndices {
    indices: Vec<NodeIndex>,
    curr: uint,
}

impl Iterator<NodeIndex> for NodeIndices {
    fn next(&mut self) -> Option<NodeIndex> {
        if self.curr < self.indices.len() {
            self.curr += 1;
            Some(self.indices[self.curr - 1])
        } else {
            None
        }
    }
}

impl FromIterator<NodeIndex> for NodeIndices {
    fn from_iter<T: Iterator<NodeIndex>>(mut it: T) -> NodeIndices {
        let mut vec = vec!();
        for i in it {
            vec.push(i);
        }
        NodeIndices { indices: vec, curr: 0 }
    }
}

impl Digraph {
    // fails when called with an invalid NodeIndex
    fn get_out_adj(&self, i: NodeIndex) -> Option<&NodeIndexSet> {
        self.out_adj.find(&i)
    }

    fn reachable(&self, i: NodeIndex) -> Option<NodeIndices> {
        match self.get_out_adj(i) {
            None => None,
            Some(set) => Some(set.iter().map(|&x| x).collect()),
        }
    }
}

fn bf_trav(g: &Digraph, start: NodeIndex) -> Tree<NodeIndex> {
    let mut tree = Tree::new();

    if g.nodes.is_empty() {
        return tree;
    }

    let mut discovered = HashSet::new();
    let mut queue = RingBuf::new();

    queue.push_back((start, None));
    discovered.insert(start);
    loop {
        match queue.pop_front() {
            None => break,
            Some((ind, parent)) => {
                match parent {
                    None => tree.add_root(ind),
                    Some(p_ind) => tree.add_child(p_ind, ind),
                }

                for i in g.reachable(ind).unwrap() {
                    if !discovered.contains(&i) {
                        queue.push_back((i, Some(ind)));
                        discovered.insert(i);
                    }
                }
            }
        }
    }
    return tree;
}


mod tree {
    use super::{NodeIndex, NodeIndexSet, HashMap, HashSet};

    struct Node<T> {
        data: T,
        index: NodeIndex,
        parent: Option<NodeIndex>,
    }

    pub struct Tree<T> {
        root: Option<NodeIndex>,
        nodes: HashMap<NodeIndex, Node<T>>,
        children: HashMap<NodeIndex, NodeIndexSet>,
        num_nodes: uint,
    }

    impl<T> Tree<T> {
        pub fn new() -> Tree<T> {
            Tree { root: None, nodes: HashMap::new(), 
                   children: HashMap::new(), num_nodes: 0 }
        }

        pub fn add_root(&mut self, val: T) {
            self.add_node(None, val);
        }

        pub fn add_child(&mut self, parent: NodeIndex, val: T) {
            self.add_node(Some(parent), val);
        }

        // Calling this function with an invalid NodeIndex will cause the program to crash.
        fn add_node(&mut self, parent: Option<NodeIndex>, val: T) -> NodeIndex {
            let ind = self.num_nodes;
            let node = Node { data: val, index: ind, parent: parent };
            self.nodes.insert(ind, node);
            self.children.insert(ind, HashSet::new());

            if parent.is_some() {
                self.children.find_mut(&parent.unwrap()).unwrap().insert(ind);
            }

            self.num_nodes += 1;
            ind
        }
    }
}
