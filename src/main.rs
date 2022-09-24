mod camera;
mod terminal;

use std::ops::{Add, Div, Mul, Sub};

use camera::OwnCamera;
use crossterm::style::{self, Attribute, Color, StyledContent, Stylize};
use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};
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
fn handle_image(term: &mut Term, image: DynamicImage) {
    let (width, height) = image.dimensions();

    term.draw(&mut |term| {
        //        for each in image.pixels().enumerate() {
        //
        //        }
        for y_pos in 0..height {
            for x_pos in 0..width {
                let pixel = image.get_pixel(x_pos, y_pos);
                term.put_pixel(x_pos, y_pos, get_ascii(&pixel));
            }
        }
    });
}

fn pixel_image(image: &mut DynamicImage, scale: u32) {
    let (width, height) = image.dimensions();

    for y_pos in (0..height).step_by(scale as usize) {
        for x_pos in (0..width).step_by(scale as usize) {
            let mut min_pixel: Option<Rgba<u8>> = None;
            for i_y_pos in y_pos..y_pos + scale {
                for i_x_pos in x_pos..x_pos + scale {
                    let pixel = image.get_pixel(i_x_pos, i_y_pos);
                    if min_pixel == None {
                        min_pixel = Some(pixel);
                    }
                    if let Some(i_min_pixel) = min_pixel {
                        if i_min_pixel[0] > pixel[0]
                            && i_min_pixel[0] > pixel[0]
                            && i_min_pixel[0] > pixel[0]
                        {
                            min_pixel = Some(pixel);
                        }
                    }
                }
            }
            if let Some(min_pixel) = min_pixel {
                for i_y_pos in y_pos..y_pos + scale {
                    for i_x_pos in x_pos..x_pos + scale {
                        image.put_pixel(i_x_pos, i_y_pos, min_pixel);
                    }
                }
            }
        }
    }
}

fn funky_image(image: &mut DynamicImage, scale: u32) {
    let (width, height) = image.dimensions();

    for y_pos in (0..height).step_by(scale as usize) {
        for x_pos in (0..width).step_by(scale as usize) {
            let mut rgb: [u8; 3] = [0; 3];
            let mut size = 0;
            for i_y_pos in y_pos..y_pos + scale {
                for i_x_pos in x_pos..x_pos + scale {
                    let pixel = image.get_pixel(i_x_pos, i_y_pos);
                    rgb[0] += pixel[0];
                    rgb[1] += pixel[1];
                    rgb[2] += pixel[2];
                    size += 1;
                }
            }
            rgb[0] = u8::min(rgb[0] / size, 255);
            rgb[1] = u8::min(rgb[1] / size, 255);
            rgb[2] = u8::min(rgb[2] / size, 255);
            for i_y_pos in y_pos..y_pos + scale {
                for i_x_pos in x_pos..x_pos + scale {
                    image.put_pixel(
                        i_x_pos,
                        i_y_pos,
                        Rgba::from_channels(rgb[0], rgb[1], rgb[2], 255),
                    );
                }
            }
        }
    }
}

fn camera_handler(term: &mut Term, camera: &mut OwnCamera) {
    loop {
        let mut frame = camera.get_frame();
        pixel_image(&mut frame, 2);
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
