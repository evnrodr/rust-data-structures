#[derive(Debug, Clone)]
pub struct Array<T> {
    pub elements: Vec<T>,
    pub size: usize,
}

impl<T: Copy> Array<T> {
    pub fn new() -> Self {
        Array {
            elements: Vec::new(),
            size: 0,
        }
    }

    pub fn insert_at_start(&mut self, element: T) {
        self.elements.resize(self.size + 1, element);
        self.size += 1;

        for i in (1..self.size).rev() {
            self.elements.swap(i, i - 1)
        }
    }

    pub fn insert_by_index(&mut self, index: usize, element: T) {
        if index > self.size + 1 {
            panic!("Index out of bounds.");
        }

        if index == 0 {
            return self.insert_at_start(element);
        }

        self.elements.resize(self.size + 1, element);
        self.size += 1;

        for i in (0..self.size).rev() {
            if i == index {
                break;
            }
            self.elements.swap(i, i - 1);
        }
    }

    pub fn insert_at_end(&mut self, element: T) {
        self.elements.resize(self.size + 1, element);
        self.size += 1;
    }

    pub fn delete_at_start(&mut self) {
        if self.size == 0 {
            panic!("Array is currently empty.")
        }

        for i in 1..self.size {
            self.elements.swap(i - 1, i)
        }

        self.elements.remove(self.size - 1);
        self.size -= 1;
    }

    pub fn delete_by_index(&mut self, index: usize) {
        if self.size == 0 {
            panic!("Array is currently empty.")
        }

        if index > self.size {
            panic!("Invalid position.");
        }

        if index == 0 {
            return self.delete_at_start();
        }

        for i in index..self.size - 1 {
            self.elements[i] = self.elements[i + 1]
        }

        self.elements.remove(self.size - 1);
        self.size -= 1;
    }

    pub fn delete_at_end(&mut self) {
        if self.size == 0 {
            panic!("Array is currently empty.")
        }

        self.elements.remove(self.size - 1);
        self.size -= 1;
    }
}
