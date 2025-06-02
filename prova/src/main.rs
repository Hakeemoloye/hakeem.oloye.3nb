//definindo a estrutura da Stack
struct Stack {
    memoria: Vec<i32>, // Vetor que armazena os elementos da pilha
    capacidade_maxima: usize
}
// Implementação da Stack
impl Stack {
    // Cria uma nova pilha com capacidade definida
    fn new(capacity: usize) -> Stack {
        Stack {
            memoria: Vec::with_capacity(capacity),
            capacidade_maxima: capacity,
        }
    }
    
    //inseriondo um elemento no topo da pilha se não estiver cheia
    fn push(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.memoria.push(value);
            true
        }
    }
    // remove e retorna o elemento do topo se existir
    fn pop(&mut self) -> Option<i32> {
        self.memoria.pop()
    }
    // Retorna uma referência ao topo da pilha, sem remover
    fn peek(&self) -> Option<&i32> {
        self.memoria.last()
    }
    // Verifica se a pilha está vazia
    fn is_empty(&self) -> bool {
        self.memoria.is_empty()
    }
    // Verifica se a pilha está cheia
    fn is_full(&self) -> bool {
        self.memoria.len() == self.capacidade_maxima
    }
    // Retorna o número de elementos na pilha
    fn len(&self) -> usize {
        self.memoria.len()
    }
    // mostra o que tem na pilha
    fn print(&self) {
        println!("Estado da pilha: {:?}", self.memoria);
    }
}
fn main() {
    // Criando uma pilha com capacidade 3
    let mut stack = Stack::new(3);
    println!("Pilha criada com capacidade de {} elementos.", stack.capacidade_maxima);
    // inserindo elementos na pilha
    stack.push(10);
    stack.push(20);
    stack.push(30);
    stack.print();
    // Tentando inserir mais um elemento, o que deve falhar
    if !stack.push(40) {
        println!("A pilha ta cheia, não foi possível inserir o elemento 40 meu chapa.");
    }
    // Exibindo o topo da pilha
    if let Some(top) = stack.peek() {
        println!("O topo da pilha é: {}", top);
    } else {
        println!("A pilha está vazia.");
    }
    // Removendo todos os elementos da pilha
    while let Some(value) = stack.pop() {
        println!("Removendo o elemento: {}", value);
        stack.print();
    }
    // Verificando se a pilha está vazia
    if stack.is_empty() {
        println!("A pilha está vazia.");
    } else {
        println!("A pilha ainda tem elementos.");
    }
}
