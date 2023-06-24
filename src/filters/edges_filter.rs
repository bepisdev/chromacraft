use crate::filters::ImageFilter;

pub struct EdgesFilter;

impl ImageFilter for EdgesFilter {
    fn apply(&self, image: image::DynamicImage) -> image::DynamicImage {
        let edges = image.filter3x3(filters::LAPLACIAN);
        image::DynamicImage::ImageRgb8(edges)
    }
}
