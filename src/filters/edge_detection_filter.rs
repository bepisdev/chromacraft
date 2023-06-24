use image::{DynamicImage, GenericImageView, Rgb, RgbImage};

use crate::filters::ImageFilter;

pub struct EdgeDetectionFilter;

impl ImageFilter for EdgeDetectionFilter {
    fn apply(&self, image: DynamicImage) -> DynamicImage {
        // Convert the image to grayscale
        let grayscale_image = image.grayscale();

        // Create an output image with the same dimensions as the input image
        let mut output_image = RgbImage::new(grayscale_image.width(), grayscale_image.height());

        // Apply the edge detection algorithm
        for y in 0..grayscale_image.height() {
            for x in 0..grayscale_image.width() {
                let pixel_value = grayscale_image.get_pixel(x, y)[0];

                // Calculate the intensity differences with neighboring pixels
                let intensity_left = if x > 0 {
                    get_intensity_difference(grayscale_image.get_pixel(x - 1, y)[0], pixel_value)
                } else {
                    0
                };

                let intensity_right = if x < grayscale_image.width() - 1 {
                    get_intensity_difference(grayscale_image.get_pixel(x + 1, y)[0], pixel_value)
                } else {
                    0
                };

                let intensity_up = if y > 0 {
                    get_intensity_difference(grayscale_image.get_pixel(x, y - 1)[0], pixel_value)
                } else {
                    0
                };

                let intensity_down = if y < grayscale_image.height() - 1 {
                    get_intensity_difference(grayscale_image.get_pixel(x, y + 1)[0], pixel_value)
                } else {
                    0
                };

                // Calculate the combined intensity difference
                let combined_intensity = intensity_left + intensity_right + intensity_up + intensity_down;

                // Set the corresponding pixel in the output image based on the intensity difference
                let edge_pixel = Rgb([combined_intensity, combined_intensity, combined_intensity]);
                output_image.put_pixel(x, y, edge_pixel);
            }
        }

        // Convert the output image to the same color space as the input image
        let output_image = match image {
            DynamicImage::ImageRgb8(_) => DynamicImage::ImageRgb8(output_image.into()),
            _ => panic!("Unsupported color space"),
        };

        output_image
    }
}

fn get_intensity_difference(value1: u8, value2: u8) -> u8 {
    value1.saturating_sub(value2)
}
