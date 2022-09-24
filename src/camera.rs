use image::{DynamicImage, ImageBuffer};
use nokhwa::{yuyv422_to_rgb888, CameraFormat, CaptureAPIBackend, FrameFormat, Camera};

pub struct OwnCamera {
    height: u32,
    width: u32,
    camera: Camera,
}

impl OwnCamera {
    pub fn new(width: u32, height: u32) -> Self {
        let mut camera = Camera::with_backend(
            0,
            Some(CameraFormat::new_from(width, height, FrameFormat::YUYV, 30)),
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

    pub fn get_frame(&mut self) -> DynamicImage {
        // let bytes = self.camera.frame_raw().expect("Stream is dead").to_vec();
        // DynamicImage::ImageRgb8(
        //     ImageBuffer::from_raw(
        //         self.width,
        //         self.height,
        //         bytes,
        //     )
        //     .unwrap(),
        // )
        DynamicImage::ImageRgb8(
            ImageBuffer::from_raw(
                self.width,
                self.height,
                yuyv422_to_rgb888(&self.camera.frame_raw().expect("Stream is dead")).expect("Conversion failed"),
            )
            .unwrap(),
        ).resize_exact(self.width >> 2, self.height >> 3, image::imageops::FilterType::Nearest)
    }
}
