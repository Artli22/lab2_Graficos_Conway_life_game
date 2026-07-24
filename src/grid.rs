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

    pub fn next_generation(&mut self) {
    let mut next_cells = vec![false; self.width * self.height];

    for y in 0..self.height {
        for x in 0..self.width {
            let neighbors = self.count_alive_neighbors(x, y);
            let currently_alive = self.is_alive(x, y);

            let will_be_alive = match (currently_alive, neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };

            let index = self.index(x, y);
            next_cells[index] = will_be_alive;
        }
    }

    self.cells = next_cells;
    }

    fn count_alive_neighbors(&self, x: usize, y: usize) -> usize {
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {

            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0
                && nx < self.width as isize
                && ny >= 0
                && ny < self.height as isize
            {
                if self.is_alive(nx as usize, ny as usize) {
                    count += 1;
                }
            }
        }
    }

        count
    }
}