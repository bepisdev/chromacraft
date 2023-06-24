use clap::{App, Arg};
use image::GenericImageView;
mod filters;

fn main() {
    let matches = App::new("Image Filter")
        .arg(
            Arg::with_name("input")
                .short('i')
                .long("input")
                .value_name("INPUT")
                .help("Sets the input image file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .help("Sets the output image file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("filter")
                .short('f')
                .long("filter")
                .value_name("FILTER")
                .help("Specifies the filter to apply (grayscale, edges, invert, brightness)")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("width")
                .short('w')
                .long("width")
                .value_name("WIDTH")
                .help("Sets the width of the output image")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("height")
                .short('h')
                .long("height")
                .value_name("HEIGHT")
                .help("Sets the height of the output image")
                .takes_value(true),
        )
        .get_matches();

    // Get the input and output file paths from command-line arguments
    let input_path = matches.value_of("input").unwrap();
    let output_path = matches.value_of("output").unwrap();

    // Load the input image using the `image` crate
    let input_image = image::open(input_path).expect("Failed to open the input image.");

    // Determine the filter based on the command-line argument
    let filter: Box<dyn filters::ImageFilter> = match matches.value_of("filter") {
        Some("grayscale") => Box::new(filters::GrayscaleFilter),
        Some("edges") => Box::new(filters::EdgesFilter),
        Some("invert") => Box::new(filters::InvertFilter),
        Some("brightness") => {
            let adjustment: i32 = matches
                .value_of("brightness")
                .expect("Missing brightness value")
                .parse()
                .expect("Invalid brightness value");
            Box::new(filters::BrightnessFilter { adjustment })
        }
        _ => panic!("Unknown filter"),
    };

    // Apply the filter to the image
    let processed_image = filter.apply(input_image);

    // Get the output dimensions from command-line arguments
    let output_width: u32 = matches
        .value_of("width")
        .map(|w| w.parse().expect("Invalid width value"))
        .unwrap_or_else(|| processed_image.width());
    let output_height: u32 = matches
        .value_of("height")
        .map(|h| h.parse().expect("Invalid height value"))
        .unwrap_or_else(|| processed_image.height());

    // Resize the image if necessary
    let resized_image = processed_image.resize_exact(output_width, output_height, image::imageops::FilterType::Lanczos3);

    // Save the processed image to the specified output file
    resized_image
        .save(output_path)
        .expect("Failed to save the output image.");
}
