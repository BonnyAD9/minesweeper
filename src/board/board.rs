use rand::{thread_rng, Rng};
use termint::{geometry::constrain::Constrain, widgets::layout::Layout};

use super::cell::Cell;

/// Struct representing board
#[derive(Debug, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    cells: Vec<Cell>,
    cur: (usize, usize),
}

impl Board {
    /// Creates new [`Board`] with given size
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::new(0x00); width * height],
            cur: (0, 0),
        }
    }

    /// Gets [`Cell`] on given position on the [`Board`]
    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(y * self.width + x)
    }

    /// Gets [`Board`] as termint Layout element
    pub fn get_element(&self) -> Layout {
        let mut layout = Layout::horizontal().center();
        layout.add_child(
            self.get_cells_layout(),
            Constrain::Length(5 * self.width),
        );
        layout
    }

    /// Changes the size of the [`Board`]
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.cells = vec![Cell::new(0x00); width * height];
    }

    /// Generates the [`Board`] - fills it with mines
    pub fn generate(&mut self, mines: usize) {
        let mut rng = thread_rng();

        let max = self.width * self.height;
        for _ in 0..mines {
            let mut rnd = rng.gen_range(0..max);
            while self.cells[rnd].get() == 0xff {
                rnd = rng.gen_range(0..max);
            }

            self.cells[rnd].set(0xff);
            self.inc_neighbors(rnd);
        }
    }

    pub fn cur_up(&mut self) {
        self.cur.1 = self.cur.1.checked_sub(1).unwrap_or(self.height - 1);
    }

    pub fn cur_down(&mut self) {
        self.cur.1 += 1;
        if self.cur.1 >= self.height {
            self.cur.1 = 0;
        }
    }

    pub fn cur_left(&mut self) {
        self.cur.0 = self.cur.0.checked_sub(1).unwrap_or(self.width - 1);
    }

    pub fn cur_right(&mut self) {
        self.cur.0 += 1;
        if self.cur.0 >= self.height {
            self.cur.0 = 0;
        }
    }
}

// Private methods implementations
impl Board {
    fn get_cells_layout(&self) -> Layout {
        let mut layout = Layout::vertical();
        for y in 0..self.height {
            let mut row = Layout::horizontal();
            for x in 0..self.width {
                let cell = if self.cur.0 == x && self.cur.1 == y {
                    self.cells[y * self.height + x].get_element_act()
                } else {
                    self.cells[y * self.height + x].get_element()
                };
                row.add_child(cell, Constrain::Length(5));
            }
            layout.add_child(row, Constrain::Length(3));
        }

        layout
    }

    /// Increments value of cell neighbors
    fn inc_neighbors(&mut self, pos: usize) {
        let x = (pos % self.width) as isize;
        let y = (pos / self.width) as isize;

        self.inc_hor_neighbors(x, y - 1);
        self.inc_hor_neighbors(x, y);
        self.inc_hor_neighbors(x, y + 1);
    }

    /// Increments value of cell horizontal neighbors
    fn inc_hor_neighbors(&mut self, x: isize, y: isize) {
        if y >= 0 && y < self.height as isize {
            self.inc_cell(x - 1, y);
            self.inc_cell(x, y);
            self.inc_cell(x + 1, y);
        }
    }

    /// Increments cell value
    fn inc_cell(&mut self, x: isize, y: isize) {
        let width = self.width as isize;
        if x >= 0 && x < width {
            self.cells[(y * width + x) as usize].inc();
        }
    }
}
