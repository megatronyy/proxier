use std::boxed::Box;

#[derive(Default)]
pub struct Tree {
    pub root:  i64,
    pub left:  Option<Box<Tree>>,
    pub right: Option<Box<Tree>>
}

impl Tree {
    pub fn new(root: i64) -> Tree {
        Tree {
            root: root,
            ..Default::default()
        }
    }

    pub fn left(mut self, leaf: Tree) -> Self {
        self.left = Some(Box::new(leaf));
        self
    }

    pub fn right(mut self, leaf: Tree) -> Self {
        self.right = Some(Box::new(leaf));
        self
    }
}

#[test]
fn test_tree() {
    let tr = Tree::new(12)
        .left(Tree::new(10).right(Tree::new(14)))
        .right(Tree::new(16).left(Tree::new(15)).right(Tree::new(22)));
    
    assert_eq!(tr.root, 12);
}