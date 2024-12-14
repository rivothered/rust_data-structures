#[derive(Debug)]
pub struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    // Inicializando a pilha
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    // Verificar se a pilha está vazia
    fn is_empty(&self) -> bool {
        0 == self.size
    }

    // Retorna o tamanho da pilha
    fn len(&self) -> usize {
        self.size
    }

    // Limpar a pilha
    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    // Adicionar item ao topo da pilha
    fn push(&mut self, item: T) {
        self.data.push(item);
        self.size += 1;
    }

    // Remover o item e retorná-lo
    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.size -= 1;
        self.data.pop()
    }

    // Retorna o item do topo da pilha sem remove-lo
    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        self.data.get(self.size-1)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }

        self.data.get_mut(self.size-1)
    }
}
