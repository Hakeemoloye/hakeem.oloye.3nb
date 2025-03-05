fn main() {
 /// Estrutura que representa a fila.
pub struct Queue<T> {
    front: Option<Box<Node<T>>>, // Ponteiro para o primeiro nó da fila
    back: *mut Node<T>,          // Ponteiro para o último nó da fila (unsafe)
    length: usize,               // Quantidade de elementos na fila
}

/// Cada nó da fila contém um valor e um ponteiro para o próximo nó.
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Queue<T> {
    /// Cria e retorna uma nova fila vazia.
    pub fn new() -> Self {
        Queue {
            front: None,
            back: std::ptr::null_mut(),
            length: 0,
        }
    }

    /// Insere um elemento no final da fila.
    pub fn enqueue(&mut self, elem: T) {
        let new_node = Box::new(Node {
            value: elem,
            next: None,
        });

        let new_node_ptr: *const Node<T> = &*new_node;

        unsafe {
            if self.back.is_null() {
                // Caso a fila esteja vazia, o novo nó é tanto o início quanto o fim.
                self.front = Some(new_node);
                self.back = new_node_ptr as *mut Node<T>;
            } else {
                // Caso contrário, adicionamos ao final.
                (*self.back).next = Some(new_node);
                self.back = new_node_ptr as *mut Node<T>;
            }
        }

        self.length += 1;
    }

    /// Remove e retorna o elemento da frente da fila.
    pub fn dequeue(&mut self) -> Option<T> {
        self.front.take().map(|node| {
            self.front = node.next;
            if self.front.is_none() {
                // Se a fila ficar vazia, o ponteiro 'back' deve ser nulo.
                self.back = std::ptr::null_mut();
            }
            self.length -= 1;
            node.value
        })
    }

    /// Retorna uma referência ao elemento da frente da fila sem removê-lo.
    pub fn peek(&self) -> Option<&T> {
        self.front.as_ref().map(|node| &node.value)
    }

    /// Retorna o número de elementos na fila.
    pub fn len(&self) -> usize {
        self.length
    }
}

impl<T> Drop for Queue<T> {
    /// Implementação do trait Drop para liberar os recursos corretamente.
    fn drop(&mut self) {
        // Liberar os nós ao desalocar a memória.
        while let Some(node) = self.front.take() {
            self.front = node.next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_operations() {
        let mut queue: Queue<i32> = Queue::new();

        // Testando enqueue
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        // Testando len
        assert_eq!(queue.len(), 3);

        // Testando peek
        assert_eq!(queue.peek(), Some(&1));

        // Testando dequeue
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.peek(), Some(&2));

        // Testando dequeue até a fila ficar vazia
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn test_empty_queue() {
        let mut queue: Queue<i32> = Queue::new();

        // Testando operações em fila vazia
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.peek(), None);
        assert_eq!(queue.len(), 0);
    }
}
}
