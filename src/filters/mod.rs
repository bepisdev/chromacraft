pub mod grayscale_filter;

pub use grayscale_filter::GrayscaleFilter;

pub trait ImageFilter {
    fn apply(&self, image: image::DynamicImage) -> image::DynamicImage;
}
