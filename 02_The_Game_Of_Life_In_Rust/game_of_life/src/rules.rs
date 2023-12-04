use crate::board::create_empty_board;

fn evolve_cell(i: u64, j: u64, board: &Vec<Vec<u8>>) -> u8 {
    // Evolve the cell at position i, j
    let cell_alive = board[i as usize][j as usize];
    let num_neighbours_alive = count_alive_neighbours(i, j, board);

    if cell_alive == 0 {
        // Potentially making cell alive again
        match num_neighbours_alive {
            3 => return 1,
            _ => return 0,
        };
    }
    match num_neighbours_alive {
        2 | 3 => 1, // Cell lives on
        _ => 0,     // Underpopulation or overpopulation
    }
}

pub fn evolve_board(board: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    // We need to freeze the state of the board
    // so that the evolve_board operates on the
    // one version of the board and consequent
    // updates do not influence each other
    let rows = board.len();
    let columns = board[0].len();
    let mut board_new_state = create_empty_board(rows as u32, columns as u32);
    for i in 0..rows {
        for j in 0..columns {
            let cell_new_state = evolve_cell(i as u64, j as u64, &board);
            board_new_state[i][j] = cell_new_state;
        }
    }
    board_new_state
}

fn count_alive_neighbours(row: u64, column: u64, board: &Vec<Vec<u8>>) -> u64 {
    // Count the number of alive neighbours
    // for a given cell
    let neighbour_indices_absolute = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut num_alive_neighbours = 0;

    for (i, j) in neighbour_indices_absolute {
        let i = i + row as i64;
        let j = j + column as i64;
        if i < 0 || j < 0 {
            continue;
        }
        let i = i as usize;
        let j = j as usize;
        if i >= board.len() || j >= board[0].len() {
            continue;
        }
        num_alive_neighbours += board[i][j] as u64;
    }
    num_alive_neighbours
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_alive_neighbours() {
        let board = vec![
            vec![0, 0, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 1],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(4, count_alive_neighbours(1, 2, &board));
    }
    #[test]
    fn test_count_alive_neighbours_edge_1() {
        let board = vec![
            vec![0, 0, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 1],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(2, count_alive_neighbours(3, 2, &board));
    }
    #[test]
    fn test_count_alive_neighbours_edge_2() {
        let board = vec![
            vec![0, 0, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 1],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(2, count_alive_neighbours(3, 2, &board));
    }
    #[test]
    fn test_evolve_cell_underpopulation() {
        let board = vec![vec![0, 1, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(0, evolve_cell(1, 1, &board));
    }
    #[test]
    fn test_evolve_cell_overpopulation() {
        let board = vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]];
        assert_eq!(0, evolve_cell(1, 1, &board));
    }
    #[test]
    fn test_evolve_cell_survival() {
        let board = vec![vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 0]];
        assert_eq!(1, evolve_cell(1, 1, &board));
    }
    #[test]
    fn test_evolve_cell_reproduction() {
        let board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![0, 1, 0]];
        assert_eq!(1, evolve_cell(1, 1, &board));
    }
    #[test]
    fn test_evolve_board() {
        let initial_state = vec![
            vec![1, 0, 0, 0, 1],
            vec![0, 1, 1, 0, 1],
            vec![0, 1, 1, 0, 0],
            vec![0, 1, 1, 0, 1],
            vec![1, 1, 0, 1, 1],
        ];
        let board_2_ground_truth = vec![
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 1, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1],
            vec![1, 1, 0, 1, 1],
        ];

        let board_3_ground_truth = vec![
            vec![0, 1, 1, 0, 0],
            vec![1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![1, 1, 0, 1, 1],
            vec![0, 0, 0, 1, 1],
        ];
        let board_4_ground_truth = vec![
            vec![0, 1, 1, 0, 0],
            vec![1, 0, 1, 0, 0],
            vec![0, 0, 0, 1, 0],
            vec![1, 1, 0, 1, 1],
            vec![0, 0, 1, 1, 1],
        ];

        let board_2 = evolve_board(initial_state);
        assert_eq!(board_2_ground_truth, board_2);
        let board_3 = evolve_board(board_2);
        assert_eq!(board_3_ground_truth, board_3);
        let board_4 = evolve_board(board_3);
        assert_eq!(board_4_ground_truth, board_4);
    }
}
