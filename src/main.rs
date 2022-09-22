mod terminal;

use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use nokhwa::{Camera, CameraFormat, CaptureAPIBackend, FrameFormat};
use std::{io, ops::Deref};
use terminal::Term;

const ASCII: [&str; 32] = [
    "Ñ", "@", "#", "W", "$", "9", "8", "7", "6", "5", "4", "3", "2", "1", "0", "?", "a", "b", "c",
    ";", ":", "+", "=", "-", ",", ".", "_", " ", " ", " ", " ", " ",
];

const ASCII_REV: [&str; 28] = [
    " ", "_", ".", ",", "-", "=", "+", ":", ";", "c", "b", "a", "?", "0", "1", "2", "3", "4", "5",
    "6", "7", "8", "9", "$", "W", "#", "@", "Ñ",
];

const ASCII_LEN: isize = ASCII.len() as isize - 1;

fn get_ascii(intensity: isize) -> &'static str {
    ASCII[(intensity) as usize]
}

fn map(number: isize, start1: isize, end1: isize, start2: isize, end2: isize) -> isize {
    start2 + (number - start1) * (end2 - start2) / (end1 - start1)
}

fn handle_image(term: &mut Term, image: DynamicImage, scale: u32) {
    let (width, height) = image.dimensions();

    term.draw(&mut |term| {
        for y_pos in 0..height {
            for x_pos in 0..width {
                // if y_pos % (scale * 2) == 0 && x_pos % scale == 0 {
                    let pixel = image.get_pixel(x_pos, y_pos);
                    let intensity = pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3;
                    let char_index = map(intensity.into(), 0, 255, ASCII_LEN, 0);
                    term.put_pixel(x_pos, y_pos, get_ascii(char_index));
                // }
            }
        }
    });
}

fn handle_image_bytes(term: &mut Term, image: &[u8], scale: usize, width: u32, height: u32) {
    term.draw(&mut |term| {
        for y_pos in (0..height).step_by(scale) {
            for x_pos in (0..width).step_by(scale) {
                let pixel_index = ((y_pos + x_pos * width) / 2) as usize;
                let avg = image[pixel_index + 0] as isize;
                let char_index = map(avg, 0, 255, 0, ASCII_LEN);
                //println!("{char_index}: {avg}");
                term.put_pixel(x_pos, y_pos, get_ascii(char_index));
                //term.put_pixel(x_pos, y_pos, &char_index.to_string());
            }
        }
    });
}

fn main() {
    let mut term = Term::new();
    let mut camera = Camera::with_backend(
        0,
        Some(CameraFormat::new_from(
            640,
            320,
            FrameFormat::YUYV,
            30,
        )),
        CaptureAPIBackend::Video4Linux,
    )
    .unwrap();
    camera.open_stream().unwrap();
    loop {
        let frame = DynamicImage::ImageRgb8(camera.frame().expect("Stream is dead")).resize(80, 60, image::imageops::FilterType::Nearest);//.resize(term.width.into(), term.height.into(), image::imageops::FilterType::Nearest);

        handle_image(&mut term, frame, 4);
    }
}
