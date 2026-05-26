// src/board/resources.rs
use bevy::prelude::*;

pub const BOARD_SIZE: usize = 4;

#[derive(Resource, Default)]
pub struct Board {
    pub tiles: [[Option<Entity>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn clear(&mut self) {
        self.tiles = [[None; BOARD_SIZE]; BOARD_SIZE];
    }
}

pub fn has_moves(grid: &[[Option<u32>; BOARD_SIZE]; BOARD_SIZE]) -> bool {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if grid[row][col].is_none() {
                return true;
            }
            let current = grid[row][col].unwrap();
            if col + 1 < BOARD_SIZE {
                if let Some(right) = grid[row][col + 1] {
                    if right == current {
                        return true;
                    }
                }
            }
            if row + 1 < BOARD_SIZE {
                if let Some(down) = grid[row + 1][col] {
                    if down == current {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn has_2048(grid: &[[Option<u32>; BOARD_SIZE]; BOARD_SIZE]) -> bool {
    grid.iter()
        .flatten()
        .any(|cell| cell.is_some_and(|v| v >= 2048))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid_from(values: [[u32; 4]; 4]) -> [[Option<u32>; 4]; 4] {
        values.map(|row| row.map(|v| if v == 0 { None } else { Some(v) }))
    }

    #[test]
    fn has_moves_empty_board() {
        let grid = [[None; 4]; 4];
        assert!(has_moves(&grid));
    }

    #[test]
    fn has_moves_full_no_merges() {
        let grid = grid_from([[2, 4, 2, 4], [4, 2, 4, 2], [2, 4, 2, 4], [4, 2, 4, 2]]);
        assert!(!has_moves(&grid));
    }

    #[test]
    fn has_moves_adjacent_same_value() {
        let grid = grid_from([
            [2, 2, 4, 8],
            [4, 8, 16, 32],
            [8, 16, 32, 64],
            [16, 32, 64, 128],
        ]);
        assert!(has_moves(&grid));
    }

    #[test]
    fn has_2048_not_present() {
        let grid = grid_from([[1024, 0, 0, 0], [0; 4], [0; 4], [0; 4]]);
        assert!(!has_2048(&grid));
    }

    #[test]
    fn has_2048_present() {
        let grid = grid_from([[2048, 0, 0, 0], [0; 4], [0; 4], [0; 4]]);
        assert!(has_2048(&grid));
    }
}
