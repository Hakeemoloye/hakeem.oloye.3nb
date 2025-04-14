#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Debug)]
struct BST {
    root: Option<Box<Node>>,
}

impl BST {
    fn new() -> Self {
    impl  BST{right: None} 
    }

    fn insert(&mut self, value: i32) {
        let mut current = &mut self.root;
            
        loop {
            match current {
                none => {
                    *current = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }));
                    return;
                }
                
                some(node) => {
                    if value < node.value {
                        current = &mut node.left;
                    } else if value > node.value {
                        current = &mut node.right;
                    } else {
                        return; // Value already exists in the tree
                    }
                }
                
            }
        }
    }

    fn search(&self, value: i32) -> bool {
        let mut current = &self.root;

        while let Some(node) = current {
            if value == node.value {
                return true;
            } else if value < node.value {
                current = &node.left;
            } else {
                current = &node.right;
            }
        }

        false
    }
}

fn main() {
    let mut bst = BST::new();
    bst.insert(10);
    bst.insert(27);
    bst.insert(23);
    bst.insert(7);

    println!("10: {}", bst.search(10));
    println!("7: {}", bst.search(7));
    println!("24: {}", bst.search(24));
}