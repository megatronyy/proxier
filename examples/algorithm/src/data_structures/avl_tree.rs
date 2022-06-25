use std::cmp::Ordering;

/// An internal node of an `AVLTree`
struct AVLNode<T: Ord> {
    value: T,
    height: usize,
    left: Option<Box<AVLNode<T>>>,
    right: Option<Box<AVLNode<T>>>,
}


/**
A set based on an AVL Tree.

An AVL Tree is a self-balancing binary search tree.
It tracks the height of each node and performs internal
rotations to maintain a height difference of at most 1
between each sibling pair.
 */
pub struct AVLTree<T: Ord> {
    root: Option<Box<AVLNode<T>>>,
    length: usize,
}

/// Refers to the left or right subtree of an `AVLNode`
#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl<T: Ord> AVLTree<T> {
    /// Creates an empty `AVLTree`
    pub fn new() -> AVLTree<T> {
        AVLTree {
            root: None,
            length: 0,
        }
    }

    /// Returns `true` if the tree contains a value.
    pub fn contains(&self, value: &T) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            current = match value.cmp(&node.value) {
                Ordering::Equal => return true,
                Ordering::Less => &node.left,
                Ordering::Greater => &node.right,
            }
        }

        false
    }
}

