macro_rules! parent {
    ($index:expr) => {
        $index >> 1
    };
}

macro_rules! left_child {
    ($index:expr) => {
        $index << 1
    };
}

macro_rules! right_child {
    ($index:expr) => {
        ($index << 1) + 1
    };
}

#[derive(Debug, Clone)]
pub struct BinaryHeap<T> {
    data: Vec<T>,
    size: usize,
}

impl<T: Default + Ord + Clone> BinaryHeap<T> {
    pub fn new() -> Self {
        BinaryHeap {
            data: vec![T::default()],
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn min(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.data[1])
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.size += 1;
        self.move_up(self.size);
    }

    fn move_up(&mut self, mut index: usize) {
        loop {
            let parent_index = parent!(index);
            if parent_index == 0 {
                break;
            }

            if self.data[parent_index] > self.data[index] {
                self.data.swap(parent_index, index);
            }

            index = parent_index;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if 0 == self.size {
            None
        } else {
            self.data.swap(1, self.size);
            let value = self.data.pop();
            self.size -= 1;
            self.move_down(1);
            value
        }
    }

    fn move_down(&mut self, mut index: usize) {
        loop {
            if left_child!(index) > self.size {
                break;
            }

            let min_child = self.min_child(index);
            if self.data[min_child] < self.data[index] {
                self.data.swap(min_child, index);
            }

            index = min_child;
        }
    }

    fn min_child(&self, index: usize) -> usize {
        let (left, right) = (left_child!(index), right_child!(index));

        if right > self.size {
            left
        } else if self.data[left] < self.data[right] {
            left
        } else {
            right
        }
    }

    pub fn build_new(&mut self, data: &Vec<T>) {
        for _ in 0..self.size {
            let _ = self.data.pop();
        }

        for d in data {
            self.push(d.clone());
        }

        self.size = data.len();

        let mut parent = parent!(self.size);
        while parent > 0 {
            self.move_down(parent);
            parent -= 1;
        }
    }

    pub fn build_add(&mut self, data: Vec<T>) {
        for value in data {
            self.push(value);
        }
    }
}
