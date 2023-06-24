use crate::filters::ImageFilter;

pub struct BrightnessFilter {
    adjustment: i32,
}

impl ImageFilter for BrightnessFilter {
    fn apply(&self, image: image::DynamicImage) -> image::DynamicImage {
        image.brighten(self.adjustment)
    }
}
