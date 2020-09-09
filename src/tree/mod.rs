pub mod format;
mod node;

use node::{TreeNode, TreeNodePtr};
use std::cmp::Ordering;

pub struct Tree<K: Ord, V> {
    root: TreeNodePtr<K, V>,
}

impl<K: Ord, V> Tree<K, V> {
    pub fn new() -> Self {
        Tree {
            root: TreeNodePtr::None,
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        self.root = Tree::put_node(&mut self.root, key, value);
        if let TreeNodePtr::Some(h) = &mut self.root {
            h.is_red = false; //Tree root is always black
        }
    }

    pub fn get(&self, search_key: K) -> Option<&V> {
        let mut current = &self.root;
        while let TreeNodePtr::Some(node) = current {
            match search_key.cmp(&node.key) {
                Ordering::Less => current = &node.left,
                Ordering::Greater => current = &node.right,
                Ordering::Equal => return Some(&node.value),
            }
        }
        None
    }

    fn put_node(node: &mut TreeNodePtr<K, V>, new_key: K, new_value: V) -> TreeNodePtr<K, V> {
        match node {
            TreeNodePtr::None => {
                return TreeNodePtr::Some(Box::new(TreeNode::create(new_key, new_value)))
            }
            TreeNodePtr::Some(node) => match new_key.cmp(&node.key) {
                Ordering::Less => {
                    node.left = Tree::put_node(&mut node.left, new_key, new_value);
                }
                Ordering::Greater => {
                    node.right = Tree::put_node(&mut node.right, new_key, new_value);
                }
                Ordering::Equal => node.value = new_value,
            },
        }
        // Red-Black Tree Balancing
        if node.right().is_red() && !node.left().is_red() {
            //Case 2
            *node = Tree::rotate_left(node.take());
        }
        if node.left().is_red() && node.left().left().is_red() {
            //Case 4
            *node = Tree::rotate_right(node.take());
        }
        if node.left().is_red() && node.right().is_red() {
            //Case 3, 4
            Tree::flip_colors(node);
        }
        node.take()
    }

    fn rotate_left(mut node: TreeNodePtr<K, V>) -> TreeNodePtr<K, V> {
        debug_assert!(node.right().is_red());
        if let TreeNodePtr::Some(h) = &mut node {
            let mut right_node = h.right.take();
            if let TreeNodePtr::Some(x) = &mut right_node {
                x.is_red = h.is_red;
                h.is_red = true;

                h.right = x.left.take();
                x.left = node;
            }
            return right_node;
        }
        node
    }

    fn rotate_right(mut node: TreeNodePtr<K, V>) -> TreeNodePtr<K, V> {
        debug_assert!(node.left().is_red());
        if let TreeNodePtr::Some(h) = &mut node {
            let mut left_node = h.left.take();
            if let TreeNodePtr::Some(x) = &mut left_node {
                x.is_red = h.is_red;
                h.is_red = true;
                h.left = x.right.take();
                x.right = node;
            }
            return left_node;
        }
        node
    }

    fn flip_colors(node: &mut TreeNodePtr<K, V>) {
        debug_assert!(!node.is_red());
        debug_assert!(node.left().is_red());
        debug_assert!(node.right().is_red());
        if let TreeNodePtr::Some(h) = node {
            h.is_red = true;
            if let TreeNodePtr::Some(left) = &mut h.left {
                left.is_red = false;
            }
            if let TreeNodePtr::Some(right) = &mut h.right {
                right.is_red = false;
            }
        }
    }
}

impl<K: Ord, V> Default for Tree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::Tree;

    #[test]
    fn when_values_added_then_only_them_present() {
        let mut tree = Tree::new();
        let max_values = 100;
        for i in 0..max_values {
            tree.put(i, i * i);
        }

        for i in 0..max_values {
            assert_eq!(Some(i * i).as_ref(), tree.get(i));
        }
        assert_eq!(None, tree.get(-1));
    }

    #[test]
    fn when_values_added_reversely_then_only_them_present() {
        let mut tree = Tree::new();
        let max_values = 100;
        for i in (0..max_values).rev() {
            tree.put(i, i * i);
        }

        for i in 0..max_values {
            assert_eq!(Some(i * i).as_ref(), tree.get(i));
        }
        assert_eq!(None, tree.get(-1));
    }

    #[test]
    fn when_values_added_alternately_then_only_them_present() {
        let mut tree = Tree::new();
        let max_values = 100;
        let mut sign = 1;
        for i in 0..max_values {
            tree.put(sign * i, i * i);
            sign *= -1;
        }

        sign = 1;
        for i in 0..max_values {
            assert_eq!(Some(i * i).as_ref(), tree.get(sign * i));
            sign *= -1;
        }
        assert_eq!(None, tree.get(max_values + 1));
    }

    #[test]
    fn when_values_added_twice_then_they_replaced() {
        let mut tree = Tree::new();
        let max_values = 100;
        for i in 0..max_values {
            tree.put(i, i * i);
        }

        for i in 0..max_values {
            tree.put(i, -i * i);
        }

        for i in 0..max_values {
            assert_eq!(Some(-i * i).as_ref(), tree.get(i));
        }
        assert_eq!(None, tree.get(-1));
    }
}
