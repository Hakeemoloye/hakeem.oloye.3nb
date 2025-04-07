fn main () {
#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Debug)]
struct BST {
    root: Option<Box<Node>>, // Nó raiz
}

impl BST {
    // Criar uma nova árvore vazia
    fn new() -> Self {
        BST { root: None }
    }

    // Verificar se a árvore está vazia
    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    // Inserir um valor na árvore
    fn insert(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            left: None,
            right: None,
        });
        
        match self.root {
            Some(ref mut root) => BST::insert_node(root, new_node),
            None => self.root = Some(new_node),
        }
    }

    // Função auxiliar para inserir um nó
    fn insert_node(node: &mut Box<Node>, new_node: Box<Node>) {
        if new_node.value < node.value {
            if let Some(ref mut left) = node.left {
                BST::insert_node(left, new_node);
            } else {
                node.left = Some(new_node);
            }
        } else if new_node.value > node.value {
            if let Some(ref mut right) = node.right {
                BST::insert_node(right, new_node);
            } else {
                node.right = Some(new_node);
            }
        }
    }

    // Buscar um valor na árvore
    fn search(&self, value: i32) -> bool {
        BST::search_node(&self.root, value)
    }

    // Função auxiliar para buscar um valor
    fn search_node(node: &Option<Box<Node>>, value: i32) -> bool {
        match node {
            Some(n) => {
                if value == n.value {
                    true
                } else if value < n.value {
                    BST::search_node(&n.left, value)
                } else {
                    BST::search_node(&n.right, value)
                }
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod bst_tests {
    use super::BST;

    #[test]
    fn test_bst_new_and_empty() {
        let bst = BST::new();
        assert!(bst.is_empty());
    }

    #[test]
    fn test_bst_insert_and_search() {
        let mut bst = BST::new();
        
        // Testando inserção de valores:)
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(18);

        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));
        assert!(bst.search(18));

        assert!(!bst.search(12));
        assert!(!bst.search(20));
        assert!(!bst.is_empty());
    }
}

}