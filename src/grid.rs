use rand::prelude::*;
use raylib::prelude::*;

pub struct Grid {
    rows: usize,
    columns: usize,
    cell_size: usize,
    cells: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new(width: u16, heigh: u16, cell_size: u16) -> Self {
        let mut grid = Self {
            rows: (heigh / cell_size).into(),
            columns: (width / cell_size).into(),
            cell_size: cell_size.into(),
            cells: vec![vec![false; (width / cell_size).into()]; (heigh / cell_size).into()],
        };
        grid.fill_random();
        grid
    }
    pub fn draw(&mut self, mut d: RaylibDrawHandle) {
        d.draw_rectangle(0, 0, 480, 480, Color::BLACK);
        for row in 0..self.rows {
            for column in 0..self.columns {
                d.draw_rectangle(
                    (column*self.cell_size+1).try_into().unwrap(),
                    (row*self.cell_size+1).try_into().unwrap(),
                    (self.cell_size-2).try_into().unwrap(),
                    (self.cell_size-2).try_into().unwrap(),
                    if self.cells[row][column] { Color::GREEN } else { Color::GRAY });

                d.draw_rectangle(
                    (column*self.cell_size).try_into().unwrap(),
                    (row*self.cell_size).try_into().unwrap(),
                    (self.cell_size).try_into().unwrap(),
                    (self.cell_size).try_into().unwrap(),
                    if self.cells[row][column] { Color::GREEN } else { Color::GRAY });
            }    
        }
    }
    pub fn fill_random(&mut self) {
        for row in 0..self.rows {
            for column in 0..self.columns {
                self.cells[row][column] = random::<bool>();
            }    
        }
    }
    pub fn set_value(&mut self, row: i32, column: i32, value: bool) {
        if self.is_within_bounds(row, column) {
            self.cells[row as usize][column as usize] = value;
        }
    }
    pub fn get_value(&mut self, row: i32, column: i32) -> bool {
        if self.is_within_bounds(row, column) {
            self.cells[row as usize][column as usize]
        }
        else {
            false
        }
    }
    fn is_within_bounds(&mut self, row: i32, column: i32) -> bool {
        (row as usize) < self.rows && (column as usize) < self.columns
    }
    pub fn get_rows(&mut self) -> i32 {
        self.rows as i32
    }
    pub fn get_columns(&mut self) -> i32 {
        self.columns as i32
    }
}
