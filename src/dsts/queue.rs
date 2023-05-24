#[derive(Debug, Clone)]

pub struct Queue<T> {
    pub elements: Vec<T>,
    pub size: usize,
}

impl<T: Copy> Queue<T> {
    pub fn new() -> Self {
        Queue {
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
            panic!("Queue is empty")
        }

        for i in 1..self.size {
            self.elements.swap(i - 1, i)
        }

        self.elements.remove(self.size - 1);
        self.size -= 1;
    }
}
