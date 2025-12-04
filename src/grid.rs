pub struct Grid2D<T> {
    width: isize,
    height: isize,
    map: Vec<T>,
}

impl<T> Grid2D<T> {
    pub fn to_index(&self, x: isize, y: isize) -> Option<usize> {
        if x < 0 || y < 0 || x > self.width || y > self.height {
            return None;
        }
        Some((y * self.width + x) as usize)
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
