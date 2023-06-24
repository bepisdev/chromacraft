pub mod grayscale_filter;
pub mod edges_filter;
pub mod invert_filter;
pub mod brightness_filter;

pub use grayscale_filter::GrayscaleFilter;
pub use edges_filter::EdgesFilter;
pub use invert_filter::InvertFilter;
pub use brightness_filter::BrightnessFilter;

pub trait ImageFilter {
    fn apply(&self, image: image::DynamicImage) -> image::DynamicImage;
}
