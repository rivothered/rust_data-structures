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

// Iterators
struct IntoIter<T> (Stack<T>);
impl <T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}

struct Iter<'a, T: 'a> { stack: Vec<&'a T>, }
impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

struct IterMut<'a, T: 'a> { stack: Vec<&'a mut T>, }
impl <'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}