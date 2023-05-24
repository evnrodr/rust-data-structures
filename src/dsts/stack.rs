#[derive(Debug, Clone)]
pub struct Stack<T> {
    pub elements: Vec<T>,
    pub size: usize,
}

impl<T: Copy> Stack<T> {
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
            size: 0,
        }
    }

    pub fn push(&mut self, element: T) {
        self.elements.resize(self.size + 1, element);
        self.size += 1;
    }

    pub fn pop(&mut self) {
        if self.size == 0 {
            panic!("Array is currently empty.")
        }
        self.elements.remove(self.size - 1);
        self.size -= 1;
    }

    pub fn is_empty(&mut self) -> bool {
        self.size == 0
    }

    pub fn peek(&mut self) -> T {
        self.elements[self.size - 1]
    }
}
