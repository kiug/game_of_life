use raylib::prelude::*;
use super::grid::*;

pub struct Simulation {
    grid: Grid,
    temp_grid: Grid,
}

impl Simulation {
    pub fn new(width: u16, heigh: u16, cell_size: u16) -> Self {
        Self {
            grid: Grid::new(width, heigh, cell_size),
            temp_grid: Grid::new(width, heigh, cell_size),
        }
    }
    pub fn draw(&mut self, d: RaylibDrawHandle) {
        self.grid.draw(d);
    }
    pub fn count_live_neighbours(&mut self, row: i32, column: i32) -> i32 {
        let mut count = 0;
        let neighbour_offsets: Vec<(i32, i32)> = vec![
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1)
            ];
        for neighbour_offset in neighbour_offsets {
            let (row_offset, column_offset) = neighbour_offset;
            let neighbour_row = (row as i32 + row_offset + self.grid.get_rows()) % self.grid.get_rows();
            let neighbour_column = (column as i32 + column_offset + self.grid.get_columns()) % self.grid.get_columns();
            if self.grid.get_value(neighbour_row.try_into().unwrap(), neighbour_column.try_into().unwrap()) {
                count = count + 1;
            }
        }
        count
    }
    pub fn update(&mut self) {
        for row in 0..self.grid.get_rows() {
            for column in 0..self.grid.get_columns() {
                let live_neighbours = self.count_live_neighbours(row, column);
                let cell_value = self.grid.get_value(row, column);
                if cell_value {
                    if live_neighbours > 3 || live_neighbours < 2 {
                        self.temp_grid.set_value(row, column, false);
                    }
                    else {
                        self.temp_grid.set_value(row, column, true);
                    }
                }
                else {
                    if live_neighbours == 3 {
                        self.temp_grid.set_value(row, column, true);
                    }
                    else {
                        self.temp_grid.set_value(row, column, false);
                    }
                }
            }    
        }
        for row in 0..self.grid.get_rows() {
            for column in 0..self.grid.get_columns() {
                self.grid.set_value(row, column, self.temp_grid.get_value(row, column));
            }
        }
    }
}