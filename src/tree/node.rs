use core::mem;

pub enum TreeNodePtr<K: Ord, V> {
    Some(Box<TreeNode<K, V>>),
    None,
}

impl<K: Ord, V> TreeNodePtr<K, V> {
    #[inline]
    pub fn take(&mut self) -> Self {
        mem::take(self)
    }

    pub fn is_red(&self) -> bool {
        match self {
            TreeNodePtr::Some(node) => node.is_red,
            _ => false,
        }
    }

    pub fn left(&self) -> &Self {
        match self {
            TreeNodePtr::Some(node) => &node.left,
            _ => self,
        }
    }

    pub fn right(&self) -> &Self {
        match self {
            TreeNodePtr::Some(node) => &node.right,
            _ => self,
        }
    }
}

impl<K: Ord, V> Default for TreeNodePtr<K, V> {
    fn default() -> Self {
        TreeNodePtr::None
    }
}

pub struct TreeNode<K: Ord, V> {
    pub key: K,
    pub value: V,
    pub left: TreeNodePtr<K, V>,
    pub right: TreeNodePtr<K, V>,
    pub is_red: bool,
}

impl<K: Ord, V> TreeNode<K, V> {
    pub fn create(key: K, value: V) -> Self {
        TreeNode {
            key,
            value,
            left: TreeNodePtr::None,
            right: TreeNodePtr::None,
            is_red: true,
        }
    }
}
