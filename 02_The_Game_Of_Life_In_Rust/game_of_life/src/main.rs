mod board;
mod display;
mod rules;
use clap::Parser;
use std::thread;

#[derive(Parser, Debug)]
struct Args {
    /// For how many iterations evolve the simulation
    #[arg(short, long, default_value_t = 100)]
    iterations: u64,

    /// The size of the square simulation board
    #[arg(short, long, default_value_t = 150)]
    board_size: u32,

    /// Sleep duration between interations
    #[arg(short, long, default_value_t = 200)]
    sleep_time_ms: u32,

    /// Initial number of randomly "turned on" cells (alive)
    #[arg(short, long, default_value_t = 2500)]
    alive_cells: u64,

    /// Seed for the initial state
    #[arg(short, long, default_value_t = 42)]
    random_seed: u8,
}

fn main() {
    let args = Args::parse();
    // Create a buffer for frames
    let mut frames = Vec::new();
    // Create the game board and randomly populate it with alive cells
    let mut game_board = board::create_empty_board(args.board_size, args.board_size);
    board::add_random_alive_cells(&mut game_board, args.alive_cells, args.random_seed);
    // Play the game of life!
    for iter in 0..args.iterations {
        // The main logic
        println!("Iteration: {}", iter);
        display::print(&game_board);
        thread::sleep_ms(args.sleep_time_ms);
        frames.push(display::create_frame(&game_board));
        print!("\x1B[2J\x1B[1;1H"); // clears the terminal
        game_board = rules::evolve_board(game_board);
    }

    display::save_gif(&frames, "game_of_life.gif");
}
