mod terminal;
mod camera;

use std::ops::{Add, Div, Mul, Sub};

use image::{DynamicImage, GenericImageView};
use terminal::Term;
use camera::OwnCamera;

const ASCII: [&str; 29] = [
    "Ñ", "@", "#", "W", "$", "9", "8", "7", "6", "5", "4", "3", "2", "1", "0", "?", "a", "b", "c",
    ";", ":", "+", "=", "-", ",", ".", "_", " ", " ",
];

const ASCII_LEN: isize = ASCII.len() as isize - 1;

/// Gets the specified ascii-character
fn get_ascii(intensity: isize) -> &'static str {
    ASCII[(intensity) as usize]
}

/// Maps a number from an old range to a new range
///
/// * number - is the number to map
/// * from - is the start-range
/// * to - is the stop-range
///
/// * returns - an T
fn map<T>(number: T, from: (T, T), to: (T, T)) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    to.0 + (number - from.0) * (to.1 - to.0) / (from.1 - from.0)
}

/// Function which handles the image to ascii-conversion
///
/// * term - is the terminal to use
/// * image - is the image to ascii-fy
fn handle_image(term: &mut Term, image: DynamicImage) {
    let (width, height) = image.dimensions();

    term.draw(&mut |term| {
        for y_pos in 0..height {
            for x_pos in 0..width {
                let pixel = image.get_pixel(x_pos, y_pos);
                let intensity = pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3;
                let char_index = map(intensity.into(), (0, 255), (ASCII_LEN, 0));
                term.put_pixel(x_pos, y_pos, get_ascii(char_index));
            }
        }
    });
}

fn main() {
    let mut term = Term::new();
    let width = u32::max(640 >> 2, term.width.into());
    let height = u32::max(480 >> 2, term.height.into());
    let mut camera = OwnCamera::new(width, height);
    camera.open_stream().unwrap();
    loop {
        let frame = camera.get_frame();

        handle_image(&mut term, frame);
    }
}
