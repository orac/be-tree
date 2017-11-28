use std::fmt;

const max_values_per_leaf: usize = 4;

struct Pivot<K, V> {
    min_key: K,
    child: Node<K, V>
}

enum Node<K, V>
{
    Branch {
        // each pair is a key and the node of stuff greater than or equal to that key; i.e. the first key is the minimum key of the vector
        pivots: Vec<Pivot<K, V>>
    },
    Leaf {
        elements: Vec<(K, V)>
    }
}

impl<K, V> Node<K, V> where K: Copy + Ord {
    fn min_key(&self) -> K {
        match *self {
            Node::Branch { pivots: ref p } => {
                p[0].min_key
            },
            Node::Leaf { elements: ref e } => {
                e[0].0
            }
        }
    }
    fn insert(&mut self, key: K, value: V) {
        match *self {
            Node::Branch { ref mut pivots } => {
                // Find a child node whose keys are not before the target key
                match pivots.iter().position(|ref p| key <= p.min_key) {
                    Some(i) => {
                        // If there is one, insert into it and update the pivot key
                        let pivot = &mut pivots[i];
                        pivot.min_key = key;
                        pivot.child.insert(key, value)
                    },
                    // o/w, insert a new leaf at the end
                    None => pivots.push(Pivot {min_key: key, child: Node::Leaf {elements: vec!((key, value))}})
                }
            }
            Node::Leaf { elements: ref mut l } => {
                let index = l.binary_search_by_key(&key, |&(k, _)| k);
                match index {
                    Err(i) => l.insert(i, (key, value)),
                    Ok(i) => l[i] = (key, value)
                }
            }
        }
    }

    fn delete(&mut self, key: K) {
        match *self {
            Node::Branch { ref mut pivots } => {
                // Find a child node whose keys are not before the target key
                match pivots.iter_mut().find(|ref p| key <= p.min_key) {
                    Some(ref mut pivot) => {
                        // If there is one, delete from it and update the pivot key
                        pivot.child.delete(key);
                        pivot.min_key = pivot.child.min_key()
                    },
                    // o/w, nothing to do
                    None => ()
                }
            }
            Node::Leaf { elements: ref mut l } if l.len() > 0 => {
                let index = l.binary_search_by_key(&key, |&(k, _)| k);
                match index {
                    Err(_) => (),
                    Ok(i) => { l.remove(i); }
                }
            }
            _ => ()
        }
    }

    fn get(&self, key: K) -> Option<&V> {
        match *self {
            Node::Branch { ref pivots } => {
                // Find a child node whose keys are not before the target key
                match pivots.iter().find(|ref p| key <= p.min_key) {
                    Some(ref pivot) => {
                        // If there is one, query it
                        pivot.child.get(key)
                    },
                    // o/w, the key doesn't exist
                    None => None
                }
            }
            Node::Leaf { elements: ref l } if l.len() > 0 => {
                let index = l.binary_search_by_key(&key, |&(k, _)| k);
                match index {
                    Err(_) => None,
                    Ok(i) => Some(&l[i].1)
                }
            }
            _ => None
        }
    }
}

/// A map based on a B𝛆-tree
pub struct BeTree< K, V > {
    root: Node< K, V >
}

impl<K, V> BeTree<K, V> where K: Copy + Ord {
    /// Create an empty B𝛆-tree.
    pub fn new() -> Self { BeTree { root: Node::Leaf { elements: Vec::new() } } }

    /// Clear the tree, removing all entries.
    pub fn clear(&mut self) {
        match self.root {
            Node::Leaf { elements: ref mut l } => l.clear(),
            _ => { self.root = Node::Leaf { elements: Vec::new() } }
        }
    }

    /// Insert a key-value pair into the tree.
    ///
    /// If the key is already present in the tree, the value is replaced. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    pub fn insert(&mut self, key: K, value: V)
    {
        self.root.insert(key, value)
    }

    /// Remove a key (and its value) from the tree.
    ///
    /// If the key is not present, silently does nothing.
    pub fn delete(&mut self, key: K)
    {
        self.root.delete(key)
    }

    /// Retrieve a reference to the value corresponding to the key.
    pub fn get(&self, key: K) -> Option<&V>
    {
        self.root.get(key)
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
        let result = b.get(0);
        assert_eq!(Some(&'x'), result);
    }

    #[test]
    fn can_insert_two() {
        let mut b = BeTree::new();
        b.insert(0, 'x');
        b.insert(-1, 'y');
        assert_eq!(Some(&'x'), b.get(0));
        assert_eq!(Some(&'y'), b.get(-1));
    }

    #[test]
    fn can_clear() {
        let mut b = BeTree::new();
        b.insert(0, 'x');
        b.insert(-1, 'y');
        b.clear();
        assert_eq!(None, b.get(0));
    }

    #[test]
    fn insert_replaces_existing() {
        let mut b = BeTree::new();
        b.insert(0, 'x');
        b.insert(0, 'y');
        assert_eq!(Some(&'y'), b.get(0));
    }

    #[test]
    fn can_delete_existing() {
        let mut b = BeTree::new();
        b.insert(0, 'x');
        b.delete(0);
        assert_eq!(b.get(0), None)
    }

    #[test]
    fn can_delete_only_existing() {
        let mut b = BeTree::new();
        b.insert(0, 'x');
        b.insert(2, 'y');
        b.delete(0);
        assert_eq!(b.get(0), None);
        assert_eq!(Some(&'y'), b.get(2));
    }

    #[test]
    fn can_delete_nothing() {
        let mut b = BeTree::<i32, char>::new();
        b.delete(0);
    }
}
