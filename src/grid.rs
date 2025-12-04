pub struct Grid2D<T> {
    pub width: isize,
    pub height: isize,
    pub map: Vec<T>,
}

impl<T> Grid2D<T> {
    pub fn to_index(&self, x: isize, y: isize) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }
        Some((y * self.width + x) as usize)
    }

    pub fn get_at_index(&self, x: isize, y: isize) -> Option<&T> {
        self.to_index(x, y).and_then(|v| self.map.get(v))
    }

    pub fn replace_at_index(&mut self, x: isize, y: isize, new_element: T) -> bool {
        self.to_index(x, y)
            .and_then(|v| self.map.get_mut(v))
            .map(|v| *v = new_element)
            .is_some()
    }

    pub fn adjacent(&self, x: isize, y: isize) -> [Option<&T>; 8] {
        [
            self.to_index(x - 1, y - 1).and_then(|v| self.map.get(v)),
            self.to_index(x, y - 1).and_then(|v| self.map.get(v)),
            self.to_index(x + 1, y - 1).and_then(|v| self.map.get(v)),
            self.to_index(x - 1, y).and_then(|v| self.map.get(v)),
            self.to_index(x + 1, y).and_then(|v| self.map.get(v)),
            self.to_index(x - 1, y + 1).and_then(|v| self.map.get(v)),
            self.to_index(x, y + 1).and_then(|v| self.map.get(v)),
            self.to_index(x + 1, y + 1).and_then(|v| self.map.get(v)),
        ]
    }
}
