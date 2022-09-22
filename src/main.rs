use eye::{hal::PlatformStream, prelude::*};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use std::io;

const ASCII: [&str; 8] = [" ", ".", ",", "~", "-", "+", "=", "@"];

fn get_ascii(intensity: u8) -> &'static str {
    ASCII[(intensity / 32) as usize]
}

fn handle_image(image: RgbImage, scale: u32) {
    let (width, height) = image.dimensions();

    for y_pos in 0..height {
        for x_pos in 0..width {
            if y_pos % (scale * 2) == 0 && x_pos % scale == 0 {
                let pixel = image.get_pixel(x_pos, y_pos);
                let intensity = if pixel[3] > 0 {
                    pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3
                } else {
                    0
                };
                print!("{}", get_ascii(intensity));
            }
        }
        if y_pos % (scale * 2) == 0 {
            println!("");
        }
    }
}

fn handle_image_bytes(image: &[u8], scale: u32, width: u32, height: u32) {
    println!("\r");
    for y_pos in 0..height {
        for x_pos in 0..width {
            if y_pos % (scale * 2) == 0 && x_pos % scale == 0 {
                let pixel = image[(y_pos * height + x_pos) as usize];
                print!("{}", get_ascii(pixel));
            }
        }
        if y_pos % (scale * 2) == 0 {
            println!("");
        }
    }
}

fn get_image(image: &str) -> DynamicImage {
    let image = image::open(image).unwrap();
    image
}

fn get_camera<'a>() -> io::Result<PlatformStream<'a>> {
    let ctx = Context::new();
    let devices = ctx.query_devices()?;
    if devices.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "No devices available"));
    }

    let dev = Device::with_uri(&devices[0])?;

    let streams = dev.query_streams()?;
    let stream_desc = streams[0].clone();
    let stream = dev.start_stream(&stream_desc)?;
    Ok(stream)
}

fn main() {
    let mut camera = get_camera().unwrap();
    loop {
        let frame = camera
            .next()
            .expect("Stream is dead")
            .expect("Failed to capture frame");
        //let image = RgbImage::from_raw(frame.width(), frame.height(), Vec::from(frame.as_bytes())).unwrap();
        //let image: ImageBuffer<Rgb<u8>, _> =
        //    ImageBuffer::from_raw(frame.width(), frame.height(), frame.as_bytes()).unwrap();
        //handle_image(image, 4);
        handle_image_bytes(frame.as_bytes(), 4, frame.width(), frame.height());
    }
    //let image = get_image("pug.png");
}
