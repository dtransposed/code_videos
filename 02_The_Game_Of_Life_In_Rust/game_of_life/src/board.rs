use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaChaRng;

pub fn create_empty_board(rows: u32, columns: u32) -> Vec<Vec<u8>> {
    // Creates an empty board of size rows x columns.
    // Empty board contains zeros of type u8.
    let mut board: Vec<Vec<u8>> = Vec::with_capacity(rows as usize);
    for _ in 0..rows {
        board.push(vec![0; columns as usize]);
    }
    board
}

pub fn add_random_alive_cells(board: &mut Vec<Vec<u8>>, num_alive_cells: u64, seed: u8) {
    // Randomly adds num_alive_cells to the board.
    // Every alive cell is represented by a 1 in the board.
    let rows = board.len();
    let columns = board[0].len();
    println!("{},{}", rows, columns);
    let seed_array = [seed; 32];
    let mut rng = ChaChaRng::from_seed(seed_array);
    let mut cells_set_to_one: u64 = 0;

    if num_alive_cells > (rows as u32 * columns as u32).into() {
        panic! {"The num_alive_cells larger than the number of cells in the board!"};
    }
    while cells_set_to_one < num_alive_cells {
        let row = rng.gen_range(0..rows);
        let column = rng.gen_range(0..columns);
        let prev_cell_state = board[row][column];

        if prev_cell_state == 0 {
            board[row][column] = 1;
            cells_set_to_one += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_empty_board() {
        let board = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(board, create_empty_board(4, 4));
    }
    #[test]
    fn test_add_random_alive_cells() {
        let expected_num_alive_cells = 5;
        let mut board = create_empty_board(10, 10);
        add_random_alive_cells(&mut board, expected_num_alive_cells, 42);

        let mut num_alive_cells = 0;

        for row in board {
            for element in row {
                num_alive_cells += element;
            }
        }

        assert_eq!(expected_num_alive_cells, num_alive_cells as u64);
    }
}
