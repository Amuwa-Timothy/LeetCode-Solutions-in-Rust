use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // Step 1: Check all 9 rows
        for row in 0..9 {
            let mut cells = Vec::new();
            for col in 0..9 {
                cells.push(board[row][col]);
            }
            if Self::has_duplicates(cells) {
                return false;
            }
        }
        
        // Step 2: Check all 9 columns
        for col in 0..9 {
            let mut cells = Vec::new();
            for row in 0..9 {
                cells.push(board[row][col]);
            }
            if Self::has_duplicates(cells) {
                return false;
            }
        }
        
        // Step 3: Check all 9 boxes
        for box_index in 0..9 {
            let box_row = box_index / 3;
            let box_col = box_index % 3;
            let start_row = box_row * 3;
            let start_col = box_col * 3;
            
            let mut cells = Vec::new();
            for row in start_row..start_row + 3 {
                for col in start_col..start_col + 3 {
                    cells.push(board[row][col]);
                }
            }
            
            if Self::has_duplicates(cells) {
                return false;
            }
        }
        
        true  
    }
    
    fn has_duplicates(cells: Vec<char>) -> bool {
        let mut seen = HashSet::new();
        for cell in cells {
            if cell == '.' {
                continue;
            } else if seen.contains(&cell) {
                return true;
            } else {
                seen.insert(cell);
            }
        }
        false
    }
}


