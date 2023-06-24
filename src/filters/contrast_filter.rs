use crate::filters::ImageFilter;

pub struct ContrastFilter {
    pub adjustment: i32,
}

impl ImageFilter for ContrastFilter {
    fn apply(&self, image: image::DynamicImage) -> image::DynamicImage {
        let contrasted_image = image.adjust_contrast(self.adjustment as f32);
        contrasted_image
    }
}
