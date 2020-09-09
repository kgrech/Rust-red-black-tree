use core::fmt;
use std::fmt::Write;

use crate::tree::node::{TreeNode, TreeNodePtr};
use crate::tree::Tree;

impl<K: Ord + fmt::Display, V> fmt::Display for Tree<K, V> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match &self.root {
            TreeNodePtr::Some(node) => write!(formatter, "digraph Tree {{\n{}\n}}", &node),
            _ => write!(formatter, "digraph Tree {{\n}}"),
        }
    }
}

fn write_leaf<K: Ord + fmt::Display, V>(str: &mut String, parent_key: &K, leaf: &TreeNode<K, V>) {
    if leaf.is_red {
        writeln!(str, "\"{}\" -> \"{}\" [color=red];", parent_key, leaf.key).unwrap();
    } else {
        writeln!(str, "\"{}\" -> \"{}\"", parent_key, leaf.key).unwrap();
    }
    write!(str, "{}", leaf).unwrap();
}

impl<K: Ord + fmt::Display, V> fmt::Display for TreeNode<K, V> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        if let TreeNodePtr::Some(leaf) = &self.left {
            write_leaf(&mut str, &self.key, leaf);
        } else {
            writeln!(str, "\"{}\" -> \"none_l_{}\"", &self.key, &self.key).unwrap();
            writeln!(str, "\"none_l_{}\"[label=\"N\\A\"];", &self.key).unwrap();
        }
        if let TreeNodePtr::Some(leaf) = &self.right {
            write_leaf(&mut str, &self.key, leaf);
        } else {
            writeln!(str, "\"{}\" -> \"none_r_{}\"", &self.key, &self.key).unwrap();
            writeln!(str, "\"none_r_{}\"[label=\"N\\A\"];", &self.key).unwrap();
        }
        formatter.write_str(str.as_str())
    }
}
