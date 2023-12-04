use gif::{Encoder, Frame};
use image::{ImageBuffer, RgbImage};
use std::fs::File;

pub fn print(board: &Vec<Vec<u8>>) {
    // Print the state of the board to the terminal.
    // For aestethic reasons, ones are printed
    // as Os and zeros are printed as "  "
    for row in board {
        println!(
            "{}",
            row.iter()
                .map(|&x| if x == 1 { "O" } else { " " })
                .collect::<Vec<&str>>()
                .join("")
        );
    }
}
pub fn create_frame(board: &Vec<Vec<u8>>) -> Frame<'static> {
    let board_size: u32 = board.len() as u32;
    let mut img: RgbImage = ImageBuffer::new(board_size, board_size);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // we need to swap x and y, different convention
        if board[y as usize][x as usize] == 1 {
            // alive pixels are white, dead pixels are black
            *pixel = image::Rgb([255, 255, 255]);
        }
    }
    Frame::from_rgb(board_size as u16, board_size as u16, &img)
}

pub fn save_gif(frames: &Vec<Frame<'static>>, file_name: &str) {
    if let Ok(file) = File::create(file_name) {
        if let Ok(mut encoder) = Encoder::new(file, frames[0].height, frames[0].width, &[]) {
            for frame in frames {
                let _ = encoder.write_frame(frame);
            }
        }
    }
}
