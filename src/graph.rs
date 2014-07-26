pub use std::collections::{RingBuf, HashMap, HashSet, Deque};

use self::tree::Tree;

mod tree {
    use super::{HashMap, HashSet};
    use std::collections::hashmap::SetItems;

    type NodeIndex = (uint, uint);
    type NodeIndexSet = HashSet<NodeIndex>;

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

        pub fn add_root(&mut self, ind: NodeIndex, val: T) -> NodeIndex {
            self.add_node(None, ind, val)
        }

        pub fn add_child(&mut self, parent: NodeIndex, ind: NodeIndex, val: T) -> NodeIndex {
            self.add_node(Some(parent), ind, val)
        }

        // Calling this function with an invalid NodeIndex will cause the program to crash.
        fn add_node(&mut self, parent: Option<NodeIndex>, ind: NodeIndex, val: T) -> NodeIndex {
            let node = Node { data: val, index: ind, parent: parent };
            self.nodes.insert(ind, node);
            self.children.insert(ind, HashSet::new());

            if parent.is_some() {
                self.children.find_mut(&parent.unwrap()).unwrap().insert(ind);
            }

            self.num_nodes += 1;
            ind
        }

        pub fn node_exists(&self, i: NodeIndex) -> bool {
            self.nodes.contains_key(&i)
        }

        pub fn path_iter<'a>(&'a self, i: NodeIndex) -> NodeValues<'a, T> {
            NodeValues { tree: self, curr: Some(i) }
        }

        pub fn ch_iter(&self, i: NodeIndex) -> Option<SetItems<NodeIndex>> {
            self.children.find(&i).map(|s| s.iter())
        }
    }

    // iterates through the node values in the path going from a node to the root.
    struct NodeValues<'a, T> {
        tree: &'a Tree<T>,
        curr: Option<NodeIndex>,
    }

    impl<'a, T> Iterator<&'a T> for NodeValues<'a, T> {
        fn next(&mut self) -> Option<&'a T> {
            let curr = match self.curr {
                None => return None,
                Some(i) => i
            };

            let node = self.tree.nodes.find(&curr).unwrap();
            self.curr = node.parent;
            Some(&node.data)
        }
    }
}


#[deriving(PartialEq, Eq, Show)]
enum EditCommand {
    Del(uint),
    Ins(uint),
    Noop,
}

/// Compute the shortest edit script that transforms x into y. This is a 
/// modification of breadth-first traversal.
///
/// Pseudocode:
/// 
/// SES(x, y):
///     (m, n) = (x.len(), y.len())
///     S = new queue
///     S.insert((0, 0))
///     mark (0, 0) as discovered
///     while S is not empty:
///         (i, j) = S.pop()
///
///         if i == m and j == n:
///             break
///
///         vec = [(i+1, j), (i, j+1)]
///         if x_{i+1} = x_{j+1} 
///         then vec.push((i+1, j+1))
///         for w in vec:
///             if w is not yet discovered
///                 S.insert(w)
///                 mark w as discovered
///
///     return the path from (0,0) to (m, n)
///
pub fn ses<T: Eq>(x: &[T], y: &[T]) -> Vec<EditCommand> {
    let (m, n) = (x.len(), y.len());
    let mut tree = Tree::new();
    let mut discovered = HashSet::new();
    let mut queue = RingBuf::new();

    // converts a pair (p, q) of coordinates into an edit command
    // that will take p -> q. fails if a single command won't work
    fn coords_to_cmd(p: (uint, uint), q: (uint, uint)) -> EditCommand {
        // TODO: explain why the numbers are right. basically, strings are 0-indexed,
        // but nodes are kind of 1-indexed
        let ((a, b), (c, d)) = (p, q);
        match (c - a, d - b) {
            (1, 1) => Noop,
            (1, 0) => Del(c - 1),
            (0, 1) => Ins(d - 1),
            _ => fail!("Cannot compute edit command for the given coords."),
        }
    }

    // called when we pop a node off the queue. since we don't generate the edit
    // graph ahead of time, we generate it on the fly.
    let add_children = |tr: &mut Tree<EditCommand>, i: uint, j: uint| {
        let coord = (i, j);
        let bot = (i+1, j);
        if i < m && !tr.node_exists(bot) {
            tr.add_child(coord, bot, Del(i));
        }

        let right = (i, j+1);
        if j < n && !tr.node_exists(right) {
            tr.add_child(coord, right, Ins(j));
        }
        println!("after bot and right");

        let diag = (i+1, j+1);
        if i < m && j < n && x[i] == y[j] && !tr.node_exists(diag) {
            tr.add_child(coord, diag, Noop);
        }
    };

    queue.push_back( ((0u, 0u), None) );
    discovered.insert((0u, 0u));
    loop {
        match queue.pop_front() {
            None => break,
            Some((coord@(i, j), parent)) => {
                match parent {
                    None => tree.add_root(coord, Noop),
                    Some(p) => {
                        tree.add_child(p, coord, coords_to_cmd(p, coord))
                    },
                };

                if i == m && j == n { break; }

                println!("i = {}, j = {}. now adding children", i, j);
                add_children(&mut tree, i, j);

                for x in tree.ch_iter(coord).unwrap() {
                    if !discovered.contains(x) {
                        queue.push_back((*x, Some(coord)));
                        discovered.insert(*x);
                    }
                }
            }
        }
    }

    tree.path_iter((m, n)).map(|&x| x).filter(|&cmd| cmd != Noop).collect()
}
