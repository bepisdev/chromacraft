use crate::filters::ImageFilter;

pub struct InvertFilter;

impl ImageFilter for InvertFilter {
    fn apply(&self, image: image::DynamicImage) -> image::DynamicImage {
        image.invert()
    }
}
