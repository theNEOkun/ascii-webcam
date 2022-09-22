mod terminal;

use std::ops::{Add, Div, Mul, Sub};

use image::{DynamicImage, GenericImageView};
use nokhwa::{Camera, CameraFormat, CaptureAPIBackend, FrameFormat};
use terminal::Term;

const ASCII: [&str; 32] = [
    "Ñ", "@", "#", "W", "$", "9", "8", "7", "6", "5", "4", "3", "2", "1", "0", "?", "a", "b", "c",
    ";", ":", "+", "=", "-", ",", ".", "_", " ", " ", " ", " ", " ",
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
    let ratio = 0.75;
    let mut camera = Camera::with_backend(
        0,
        Some(CameraFormat::new_from(
            map(640, (0, 640), (0, f32::ceil(term.width as f32 * ratio) as u32)), //(term.width / 2).into(),
            map(480, (0, 480), (0, term.height.into())),
            FrameFormat::YUYV,
            30,
        )),
        CaptureAPIBackend::Video4Linux,
    )
    .unwrap();
    camera.open_stream().unwrap();
    loop {
        let frame = DynamicImage::ImageRgb8(camera.frame().expect("Stream is dead"));
        // .resize(
        // (term.width / 2).into(),
        // term.height.into(),
        // image::imageops::FilterType::Nearest);

        handle_image(&mut term, frame);
    }
}
