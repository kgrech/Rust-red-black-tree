pub type TreeNodePtr<K, V> = Option<Box<TreeNode<K, V>>>;

pub struct TreeNode<K: Ord, V> {
    pub key: K,
    pub value: V,
    pub left: TreeNodePtr<K, V>,
    pub right: TreeNodePtr<K, V>
}

impl<K: Ord, V> TreeNode<K, V> {
    pub fn create(key: K, value: V) -> Self {
        TreeNode {
            key,
            value,
            left: None,
            right: None
        }
    }
}
