# Color Gradient Generator

A simple Rust command-line tool for generating and displaying a gradient of colors between two hex color values in the terminal.

## Features

- Converts hex colors (e.g., `#FF5733`) to RGB values and vice versa.
- Generates a gradient of colors between two hex color codes.
- Prints each color in the gradient as a color block in the terminal.

## Getting Started

### Prerequisites

- **Rust**: Make sure you have Rust installed. You can install Rust via [rustup](https://rustup.rs/).

### Installation

Clone this repository and navigate into the project directory:

```bash
git clone https://github.com/V-Karch/gradient-maker.git
cd gradient-maker
```

Build the project with:

```bash
cargo build --release
```

## Usage

Run the program with the following command:

```bash
cargo run "<start_color>" "<end_color>" [steps]
```

### Arguments

- `<start_color>`: The hex code of the starting color (e.g., `#FF5733`).
- `<end_color>`: The hex code of the ending color (e.g., `#33A1FF`).
- `[steps]` (optional): The number of gradient steps (default is 5). Must be an integer greater than or equal to 2.

### Example

Generate a gradient from red to blue with 10 steps:

```bash
cargo run "#FF0000" "#0000FF" 10
```

This command will display a color gradient from red to blue in 10 steps in your terminal.

## Project Structure

- `hex_to_rgb`: Converts a hex color code to an RGB tuple. Returns `None` if the input is invalid.
- `rgb_to_hex`: Converts RGB values to a hex color string.
- `color_gradient`: Creates a gradient from the start color to the end color over the specified number of steps. Returns a list of hex and RGB tuples.
- `print_color_in_terminal`: Prints a single color as a block in the terminal using ANSI escape codes.
- `main`: Parses command-line arguments, generates the color gradient, and displays each color.

## Example Outputs

![example]("images/Example.png)


Each line shows a color block followed by the corresponding hex code.

## Error Handling

The program validates the format of hex color codes. It returns an error message if the inputs are not valid hex codes (e.g., if they do not start with `#` or are not 7 characters long).

## License

This project is open-source and available under the [MIT License](LICENSE).
