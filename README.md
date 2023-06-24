Certainly! Here's an updated version of the README.md file with the list of available filters (grayscale, brightness, and invert) and the usage instructions for the adjustment flag of the brightness filter:

```markdown
# ChromaCraft

ChromaCraft is a command-line image processing tool that allows you to apply various filters to your images. It is built using Rust and provides a simple way to enhance and modify images.

## Available Filters

- Grayscale: Converts the image to grayscale.
- Brightness: Adjusts the brightness of the image.
- Invert: Inverts the pixels of the image.

## Usage

To run ChromaCraft, you need to have Rust installed on your system. Follow the official Rust installation guide at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) to install Rust.

### Command-line Arguments

ChromaCraft accepts the following command-line arguments:

- `-i`, `--input`: Sets the input image file path.
- `-o`, `--output`: Sets the output image file path.
- `-f`, `--filter`: Specifies the filter to apply. Available options: `grayscale`, `brightness`, `invert`.
- `-a`, `--adjustment`: Adjustment value for the brightness filter (optional).
- `-w`, `--width`: Sets the width of the output image (optional).
- `-h`, `--height`: Sets the height of the output image (optional).

### Examples

Apply the grayscale filter to an image:

```bash
$ chromacraft -i input.jpg -o output.jpg -f grayscale
```

Adjust the brightness of an image by specifying the adjustment value:

```bash
$ chromacraft -i input.jpg -o output.jpg -f brightness -a 50
```

Invert the pixels of an image:

```bash
$ chromacraft -i input.jpg -o output.jpg -f invert
```

Resize an image by specifying the width and height:

```bash
$ chromacraft -i input.jpg -o output.jpg -w 800 -h 600
```

If you don't specify the width and height, the output image will have the same dimensions as the processed image.

## License

This project is licensed under the [MIT License](LICENSE).
```

Feel free to adjust the content as needed. Let me know if there's anything else I can assist you with!
