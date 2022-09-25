use std::ops::{Deref, DerefMut};

use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};

pub struct ImageStruct {
    image: DynamicImage,
    width: u32,
    height: u32,
}

impl ImageStruct {
    pub fn new(image: DynamicImage) -> Self {
        let (width, height) = image.dimensions();
        Self {
            image,
            width,
            height,
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn pixel_image(&mut self, scale: u32) {
        for y_pos in (0..self.height).step_by(scale as usize) {
            for x_pos in (0..self.width).step_by(scale as usize) {
                let mut min_pixel: Option<Rgba<u8>> = None;

                for i_y_pos in y_pos..y_pos + scale {
                    for i_x_pos in x_pos..x_pos + scale {
                        let pixel = self.image.get_pixel(i_x_pos, i_y_pos);
                        if min_pixel == None {
                            min_pixel = Some(pixel);
                        } else {
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
                }

                if let Some(min_pixel) = min_pixel {
                    for i_y_pos in y_pos..y_pos + scale {
                        for i_x_pos in x_pos..x_pos + scale {
                            self.put_pixel(i_x_pos, i_y_pos, min_pixel);
                        }
                    }
                }
            }
        }
    }

    pub fn funky_image(&mut self, scale: u32) {
        for y_pos in (0..self.height).step_by(scale as usize) {
            for x_pos in (0..self.width).step_by(scale as usize) {
                let mut rgb: [u8; 4] = [0; 4];
                let mut size = 0;
                for i_y_pos in y_pos..y_pos + scale {
                    for i_x_pos in x_pos..x_pos + scale {
                        let pixel = self.get_pixel(i_x_pos, i_y_pos);
                        rgb[0] += pixel[0];
                        rgb[1] += pixel[1];
                        rgb[2] += pixel[2];
                        size += 1;
                    }
                }
                rgb[0] = u8::min(rgb[0] / size, 255);
                rgb[1] = u8::min(rgb[1] / size, 255);
                rgb[2] = u8::min(rgb[2] / size, 255);
                rgb[4] = 255;
                for i_y_pos in y_pos..y_pos + scale {
                    for i_x_pos in x_pos..x_pos + scale {
                        self.image.put_pixel(
                            i_x_pos,
                            i_y_pos,
                            *Rgba::from_slice(&rgb),
                        );
                    }
                }
            }
        }
    }
}

impl Deref for ImageStruct {
    type Target = DynamicImage;

    fn deref(&self) -> &Self::Target {
        &self.image
    }
}

impl DerefMut for ImageStruct {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.image
    }
}
