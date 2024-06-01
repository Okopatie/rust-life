#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
enum CellState {
    Alive,
    Dead,
}


#[derive(Copy, Clone, Debug)]
pub struct Cell {
    state: CellState,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self.state {
            CellState::Alive => "#",
            CellState::Dead => "." ,
        };
        write!(f, "{}", output)
    }
}

impl Cell {
    fn new_alive() -> Self {
        Self {
            state: CellState::Alive,
        }
    }
    pub fn new_dead() -> Self {
        Self {
            state: CellState::Dead,
        }
    }
    pub fn is_dead(&self) -> bool {
        if let CellState::Dead = self.state {
            true
        } else {
            false
        }
    }
    pub fn is_alive(&self) -> bool {
        if let CellState::Alive = self.state {
            true
        } else {
            false
        }
    }
    fn make_alive(&mut self) {
        self.state = CellState::Alive;
    }
    fn make_dead(&mut self) {
        self.state = CellState::Dead;
    }
    fn toggle(&mut self) {
        match self.state {
            CellState::Alive => self.make_dead(),
            CellState::Dead => self.make_alive(),
        };
    }
    fn update(&mut self, neighbours: Vec<Option<Cell>>) {
        let num_neighbours = neighbours
            .iter()
            .filter(|x| x.is_some())
            .filter(|x| x.unwrap().is_alive())
            .count();
        self.state = if self.is_alive() {
            if num_neighbours <= 3 && num_neighbours >= 2 {
                CellState::Alive
            } else {
                CellState::Dead
            }
        } else {
            if num_neighbours == 3 {
                CellState::Alive
            } else {
                CellState::Dead
            }
        };
    }
}

#[derive(Clone)]
pub struct Board {
    pub board: Vec<Cell>,
    pub x_dim: usize,
    pub y_dim: usize,
}

impl Board {
    pub fn new(x_dim: usize, y_dim: usize) -> Self {
        let num_cells = x_dim * y_dim;
        let mut board = Vec::new();
        for i in 0..num_cells {
            board.push(Cell::new_dead());
        }
        Self {
            board,
            x_dim,
            y_dim
        }
    }
    pub fn reset(&mut self) {
        let length = self.x_dim * self.y_dim;
        let mut new_board = vec![];
        for i in 0..length {
            new_board.push(Cell::new_dead());
        }
        self.board = new_board;
    }
    pub fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        if x >= self.x_dim || x < 0 || y >= self.y_dim || y < 0 {
            None
        } else {
            let index = y * self.x_dim + x;
            Some(self.board[index])
        }
    }
    pub fn update_board(&mut self) {
        let board_copy = self.clone();
        for x in 0..self.x_dim {
            for y in 0..self.y_dim {
                let neighbours = vec![
                    board_copy.get_cell(x-1, y-1),
                    board_copy.get_cell(x-1, y),
                    board_copy.get_cell(x-1, y+1),
                    board_copy.get_cell(x, y-1),
                    board_copy.get_cell(x, y+1),
                    board_copy.get_cell(x+1, y-1),
                    board_copy.get_cell(x+1, y),
                    board_copy.get_cell(x+1, y+1)
                ];
                let index = y * self.x_dim + x;
                self.board[index].update(neighbours);
            }
        }
    }
    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        let index = y * self.x_dim + x;
        self.board[index].toggle();
    }
    pub fn draw(&self) {
        for y in 0..self.y_dim {
            for x in 0..self.x_dim {
                let cell = self.get_cell(x, y).unwrap();
                print!("{}", cell);
            }
            print!("\n");
        }
    }
    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.x_dim + x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn initialise_neighbours_for_testing(num_neighbours: usize) -> Vec<Option<Cell>> {
        (0..8).map(|x| {
            if x < num_neighbours {
                Some(Cell::new_alive())
            } else {
                Some(Cell::new_dead())
         }
        })
            .collect()
    }
    #[test]
    fn test_dying() {
        let mut cell = Cell::new_alive();
        let neighbours = initialise_neighbours_for_testing(4);
        cell.update(neighbours);
        assert!(cell.is_dead());
        let mut cell = Cell::new_alive();
        let neighbours = initialise_neighbours_for_testing(1);
        cell.update(neighbours);
        assert!(cell.is_dead());
    }

    #[test]
    fn test_stay_alive() {
        for i in 2..=3 {
            let mut cell = Cell::new_alive();
            let neighbours = initialise_neighbours_for_testing(i);
            cell.update(neighbours);
            assert!(cell.is_alive())
        }
    }

    #[test]
    fn test_reproduce() {
        let mut cell = Cell::new_dead();
        let neighbours = initialise_neighbours_for_testing(3);
        cell.update(neighbours);
        assert!(cell.is_alive());
    }
}











