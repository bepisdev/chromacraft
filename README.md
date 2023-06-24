## ChromaCraft

ChromaCraft is a command-line tool for image processing and filtering built with Rust. It provides a collection of filters that can be applied to images to enhance, modify, or transform them. With ChromaCraft, you can easily manipulate images and unleash your creativity.

### Usage

```plaintext
chromacraft --input <INPUT> --output <OUTPUT> --filter <FILTER> [--width <WIDTH>] [--height <HEIGHT>]
```

- `<INPUT>`: Specifies the path to the input image file.
- `<OUTPUT>`: Specifies the path to save the output image file.
- `<FILTER>`: Specifies the filter to apply to the image. Available filters are:
  - `grayscale`: Converts the image to grayscale.
  - `edges`: Finds and highlights edges in the image.
  - `invert`: Inverts the colors of the image.
  - `brightness <VALUE>`: Adjusts the brightness of the image by adding the specified value to each pixel. `<VALUE>` can be positive or negative.
- `--width <WIDTH>` (optional): Specifies the desired width of the output image.
- `--height <HEIGHT>` (optional): Specifies the desired height of the output image.

If the `--width` or `--height` options are not provided, ChromaCraft will use the original dimensions of the input image for the output.

### Examples

Convert an image to grayscale:

```plaintext
chromacraft --input input.png --output output.png --filter grayscale
```
### Note

The dimensions specified using `--width` and `--height` should maintain the aspect ratio of the image to avoid distortion.

## Contributing

Contributions to ChromaCraft are welcome! If you have any ideas, improvements, or bug fixes, feel free to open an issue or submit a pull request on the [GitHub repository](https://github.com/joshburnsxyz/chromacraft).

## License

ChromaCraft is released under the [MIT License](https://opensource.org/licenses/MIT).
