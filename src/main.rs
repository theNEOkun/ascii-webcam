mod camera;
mod terminal;
mod image_struct;

use std::ops::{Add, Div, Mul, Sub};

use camera::OwnCamera;
use crossterm::style::{self, Attribute, Color, StyledContent, Stylize};
use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};
use image_struct::ImageStruct;
use terminal::Term;

const ASCII: [&str; 29] = [
    "Ñ", "@", "#", "W", "$", "9", "8", "7", "6", "5", "4", "3", "2", "1", "0", "?", "a", "b", "c",
    ";", ":", "+", "=", "-", ",", ".", "_", " ", " ",
];

const SPACE: &str = " ";

const ASCII_LEN: isize = ASCII.len() as isize - 1;

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

/// Gets the specified ascii-character
fn get_ascii_styled(pixel: &Rgba<u8>) -> StyledContent<&'static str> {
    //let intensity = pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3;
    //let char_index = map(intensity.into(), (0, 255), (ASCII_LEN, 0)) as usize;
    SPACE.on(Color::Rgb {
        r: cl(pixel[0]),
        g: cl(pixel[1]),
        b: cl(pixel[2]),
    })
    //ASCII[char_index].white()
}

fn cl(color: u8) -> u8 {
    color
}

/// Gets the specified ascii-character
fn get_ascii(pixel: &Rgba<u8>) -> &'static str {
    let intensity = pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3;
    let char_index = map(intensity.into(), (0, 255), (ASCII_LEN, 0)) as usize;
    ASCII[char_index]
}

/// Function which handles the image to ascii-conversion
///
/// * term - is the terminal to use
/// * image - is the image to ascii-fy
fn handle_image(term: &mut Term, image: ImageStruct) {
    let (width, height) = image.dimensions();

    term.draw(&mut |term| {
        for y_pos in 0..height {
            for x_pos in 0..width {
                let pixel = image.get_pixel(x_pos, y_pos);
                term.put_pixel_styled(x_pos, y_pos, &get_ascii_styled(&pixel));
            }
        }
    });
}

fn camera_handler(term: &mut Term, camera: &mut OwnCamera) {
    loop {
        let mut frame = camera.get_frame();
        frame.pixel_image(2);
        handle_image(term, frame);
    }
}

fn main() {
    let mut term = Term::new();
    let width = 640;
    let height = 480;
    let mut camera = OwnCamera::new(width, height);
    camera_handler(&mut term, &mut camera);
}
