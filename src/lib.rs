#[derive(PartialEq, Debug)]
pub enum BTree<K,V> where K: PartialEq+Ord{
    Empty,
    Node {
        key: K,
        val: V,
        left: Box<BTree<K,V>>,
        right: Box<BTree<K,V>>,
    }
}

impl<K,V> BTree<K,V> where K: Ord{
    pub fn node(key: K, val: V) -> Self{
        BTree::Node {
            key: key,
            val: val,
            left: Box::new(BTree::Empty),
            right: Box::new(BTree::Empty),
        }
    }

    pub fn insert(&mut self, key: K, val: V){
        match *self{
            BTree::Empty => *self =  BTree::node(key, val),
            BTree::Node {key: ref k0, val: ref mut v0, ref mut left, ref mut right} => {
                if key < *k0{
                    left.insert(key, val);
                } else if key > *k0 {
                    right.insert(key, val);
                } else {
                    *v0 = val;
                }
            }
        }
    }
    pub fn find<'a >(&'a self, key: &K) -> Option<&'a V>{
        match *self {
            BTree::Empty => None,
            BTree::Node { key: ref k0, ref val, ref left, ref right } => {
                if key < k0{
                    left.find(key)
                } else if key > k0{
                    right.find(key)
                } else {
                    Some(val)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BTree;
    #[test]
    fn test_insert_into_empty(){
        let mut tree = BTree::Empty;
        tree.insert(1, 42);
        assert_eq!(tree, BTree::node(1, 42));
    }

    #[test]
    fn test_insert_into_left(){
        let mut tree = BTree::node(1, 42);
        tree.insert(0, 41);

        assert_eq!(tree, BTree::Node {
            key: 1,
            val: 42,
            left: Box::new(BTree::node(0,41)),
            right: Box::new(BTree::Empty),
        })
    }
    #[test]
    fn test_find_nothing(){
        let tree : BTree<i32, ()> = BTree::Empty;
        assert_eq!(tree.find(&1), None);
    }
    #[test]
    fn test_dont_find(){
        let tree = BTree::node(1, 42);
        assert_eq!(tree.find(&13), None);
    }

    #[test]
    fn test_find_child(){
        let mut tree = BTree::node (1, 42);
        tree.insert(2, 43);
        assert_eq!(tree.find(&2), Some(&43));
    }
}
