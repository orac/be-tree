const max_values_per_leaf: usize = 4;

enum Node<K, V>
{
    Leaf
    {
        elements: Vec<(K, V)>
    }
}

struct BeTree<K, V>
{
    root: Node<K, V>
}

impl<K, V> BeTree<K, V> where V: Copy, K: Copy + Ord {
    fn new() -> Self { BeTree { root: Node::Leaf { elements: Vec::new() }} }
    fn insert(&mut self, key: K, value: V)
    {
        match self.root
        {
            Node::Leaf {elements: ref mut l } => {
                let index = l.binary_search_by_key(&key, |&x| x.0);
                match index {
                    Err(i) => l.insert(i, (key, value)),
                    Ok(_) => ()
                }
            }
        }
    }

    fn delete(&mut self, key: K)
    {
        match self.root
            {
                Node::Leaf {elements: ref mut l } if l.len() > 0 => {
                    let index = l.binary_search_by_key(&key, |&x| x.0);
                    match index {
                        Err(_) => (),
                        Ok(i) => {l.remove(i);}
                    }
                },
                _ => ()
            }
    }

    fn query(&self, key: K) -> Option<V>
    {
        match self.root
            {
                Node::Leaf {elements: ref l } if l.len() > 0 => {
                    let index = l.binary_search_by_key(&key, |&x| x.0);
                    match index {
                        Err(_) => None,
                        Ok(i) => Some(l[i].1)
                    }
                },
                _ => None
            }
    }
}

#[cfg(test)]
mod tests {
    use BeTree;
    #[test]
    fn can_construct() {
        BeTree::<i32, char>::new();
    }

    #[test]
    fn can_insert_single() {
        let mut b = BeTree::new();
        b.insert(0, 'x');
        let result = b.query(0);
        assert_eq!(Some('x'), result);
    }

    #[test]
    fn can_insert_two() {
        let mut b = BeTree::new();
        b.insert(0, 'x');
        b.insert(-1, 'y');
        assert_eq!(Some('x'), b.query(0));
        assert_eq!(Some('y'), b.query(-1));
    }

    #[test]
    fn can_delete_existing() {
        let mut b = BeTree::new();
        b.insert(0, 'x');
        b.delete(0);
        assert_eq!(b.query(0), None)
    }

    #[test]
    fn can_delete_only_existing() {
        let mut b = BeTree::new();
        b.insert(0, 'x');
        b.insert(2, 'y');
        b.delete(0);
        assert_eq!(b.query(0), None);
        assert_eq!(Some('y'), b.query(2));
    }

    #[test]
    fn can_delete_nothing() {
        let mut b = BeTree::<i32, char>::new();
        b.delete(0);
    }
}
