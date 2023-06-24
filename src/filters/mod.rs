pub trait ImageFilter {
    fn apply(&self, image: image::DynamicImage) -> image::DynamicImage;
}
