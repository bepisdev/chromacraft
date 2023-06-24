pub mod grayscale_filter;
pub mod brightness_filter;
pub mod invert_filter;

pub use grayscale_filter::GrayscaleFilter;
pub use brightness_filter::BrightnessFilter;
pub use invert_filter::InvertFilter;

pub trait ImageFilter {
    fn apply(&self, image: image::DynamicImage) -> image::DynamicImage;
}
