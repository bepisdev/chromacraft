use crate::filters::ImageFilter;

pub struct GrayscaleFilter;

impl ImageFilter for GrayscaleFilter {
    fn apply(&self, image: image::DynamicImage) -> image::DynamicImage {
        image.grayscale()
    }
}
