pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        self.cells[self.index(x, y)]
    }

    pub fn set_alive(
        &mut self,
        x: usize,
        y: usize,
        alive: bool,
    ) {
        if x >= self.width || y >= self.height {
            return;
        }

        let index = self.index(x, y);
        self.cells[index] = alive;
    }

    pub fn clear(&mut self) {
        self.cells.fill(false);
    }

    pub fn place_pattern(
        &mut self,
        pattern: &[&str],
        start_x: usize,
        start_y: usize,
    ) {
        for (pattern_y, row) in pattern.iter().enumerate() {
            for (pattern_x, symbol) in row.chars().enumerate() {
                if symbol == 'O' {
                    self.set_alive(
                        start_x + pattern_x,
                        start_y + pattern_y,
                        true,
                    );
                }
            }
        }
    } 
}