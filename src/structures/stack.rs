#[derive(Debug)]
pub struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

#[allow(dead_code)]
impl<T> Stack<T> {
    // Inicializando a pilha
    pub fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    // Verificar se a pilha está vazia
    pub fn is_empty(&self) -> bool {
        0 == self.size
    }

    // Retorna o tamanho da pilha
    pub fn len(&self) -> usize {
        self.size
    }

    // Limpar a pilha
    pub fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    // Adicionar item ao topo da pilha
    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.size += 1;
    }

    // Remover o item e retorná-lo
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.size -= 1;
        self.data.pop()
    }

    // Retorna o item do topo da pilha sem remove-lo
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        self.data.get(self.size-1)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }

        self.data.get_mut(self.size-1)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }

        iterator
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }

        iterator
    }
}

// Iterators
pub struct IntoIter<T> (Stack<T>);
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

pub struct Iter<'a, T: 'a> { stack: Vec<&'a T>, }
impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

pub struct IterMut<'a, T: 'a> { stack: Vec<&'a mut T>, }
impl <'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}