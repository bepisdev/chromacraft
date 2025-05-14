# ChromaCraft

ChromaCraft is a command-line image processing tool that allows you to apply various filters to your images. It is built using Rust and provides a simple way to enhance and modify images.

## Available Filters

- Grayscale: Converts the image to grayscale.
- Brightness: Adjusts the brightness of the image.
- Contrast: Adjusts the contrast of the image.
- Invert: Inverts the pixels of the image.
- Edges: Detects edges in the image.

## Usage

### Command-line Arguments

ChromaCraft accepts the following command-line arguments:

- `-i`, `--input`: Sets the input image file path.
- `-o`, `--output`: Sets the output image file path.
- `-f`, `--filter`: Specifies the filter to apply. Available options: `grayscale`, `brightness`, `contrast`, `invert`, `edges`.
- `-a`, `--adjustment`: Adjustment value for the brightness and contrast filters (optional).
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

Adjust the contrast of an image by specifying the adjustment value:

```bash
$ chromacraft -i input.jpg -o output.jpg -f contrast -a 1.5
```

Invert the pixels of an image:

```bash
$ chromacraft -i input.jpg -o output.jpg -f invert
```

Detect edges in an image:

```bash
$ chromacraft -i input.jpg -o output.jpg -f edges
```

Resize an image by specifying the width and height:

```bash
$ chromacraft -i input.jpg -o output.jpg -w 800 -h 600
```

If you don't specify the width and height, the output image will have the same dimensions as the processed image.

## installation

### Prerequisites

To run ChromaCraft, you need to have Rust installed on your system. If you haven't installed Rust, you can do so by following the official Rust installation guide:

1. Visit [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2. Follow the instructions provided to install Rust.

### Building from Source

To build ChromaCraft from source, follow these steps:

1. Clone the ChromaCraft repository from GitHub using Git:

```bash
$ git clone https://github.com/bepisdev/chromacraft.git
```

2. Navigate to the project directory:
```bash
$ cd chromacraft
```

3. Build the project using Cargo

 ``` bash
 $ cargo build --release
 ```

4. Copy the built program onto your `$PATH`

``` bash
sudo cp ./target/release/chromacraft /usr/bin/chromacraft
```

 

## License

This project is licensed under the [MIT License](LICENSE).
