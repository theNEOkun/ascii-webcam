use image::{DynamicImage, ImageBuffer};
use nokhwa::{yuyv422_to_rgb888, Camera, CameraFormat, CaptureAPIBackend, FrameFormat};

use crate::image_struct::ImageStruct;

pub struct OwnCamera {
    height: u32,
    width: u32,
    camera: Camera,
}

impl OwnCamera {
    pub fn new(width: u32, height: u32) -> Self {
        let mut camera = Camera::with_backend(
            0,
            Some(CameraFormat::new_from(width, height, FrameFormat::MJPEG, 30)),
            CaptureAPIBackend::Video4Linux,
        )
        .unwrap();
        camera.open_stream().unwrap();
        Self {
            width,
            height,
            camera,
        }
    }

    pub fn get_frame(&mut self) -> ImageStruct {
        ImageStruct::new(
            DynamicImage::ImageRgb8(
                ImageBuffer::from_raw(
                    self.width,
                    self.height,
                    yuyv422_to_rgb888(&*self.camera.frame_raw().expect("Stream is dead")).unwrap(),
                )
                .unwrap(),
            )
            .resize_exact(
                self.width >> 2,
                self.height >> 2,
                image::imageops::FilterType::Nearest,
            ),
        )
    }
}
