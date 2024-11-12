use std::env;

// Convert a hex color code (e.g., "#FF5733") to an RGB tuple (e.g., (255, 87, 51))
// Returns None if the input is not a valid 7-character hex color starting with '#'
fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return None;
    }

    // Parse each color component (red, green, blue) from the hex string
    let r: u8 = u8::from_str_radix(&hex[1..3], 16).ok()?;
    let g: u8 = u8::from_str_radix(&hex[3..5], 16).ok()?;
    let b: u8 = u8::from_str_radix(&hex[5..7], 16).ok()?;

    return Some((r, g, b));
}

// Convert RGB values (e.g., 255, 87, 51) back into a hex color string (e.g., "#FF5733")
fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    // Format each component as a two-character hexadecimal string and combine
    return format!("#{:02X}{:02X}{:02X}", r, g, b);
}

// Generate a gradient of colors between two hex colors, divided into a specified number of steps
// Returns a vector of tuples containing hex color strings and their RGB tuples
fn color_gradient(
    hex_start: &str,
    hex_end: &str,
    steps: usize,
) -> Option<Vec<(String, (u8, u8, u8))>> {
    // Ensure at least 2 steps; default to 5 if fewer steps are given
    let steps = if steps < 2 { 5 } else { steps };

    // Convert start and end hex colors to RGB
    let (start_r, start_g, start_b) = hex_to_rgb(hex_start)?;
    let (end_r, end_g, end_b) = hex_to_rgb(hex_end)?;

    let mut gradient = Vec::with_capacity(steps);
    for i in 0..steps {
        // Calculate the ratio of the current step position
        let ratio = i as f32 / (steps - 1) as f32;

        // Interpolate each RGB component between the start and end colors
        let r = start_r as f32 + (end_r as f32 - start_r as f32) * ratio;
        let g = start_g as f32 + (end_g as f32 - start_g as f32) * ratio;
        let b = start_b as f32 + (end_b as f32 - start_b as f32) * ratio;

        // Convert interpolated RGB values to hex and add to the gradient list
        let color_hex = rgb_to_hex(r.round() as u8, g.round() as u8, b.round() as u8);
        gradient.push((
            color_hex,
            (r.round() as u8, g.round() as u8, b.round() as u8),
        ));
    }

    return Some(gradient);
}

// Print a color block in the terminal using ANSI escape codes to represent an RGB color
fn print_color_in_terminal(hex: &str, rgb: (u8, u8, u8)) {
    // Display a colored block followed by the hex color code
    let (r, g, b) = rgb;
    println!("\x1b[48;2;{};{};{}m  \x1b[0m {} ({}, {}, {})", r, g, b, hex, r, g, b);
}

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Validate arguments, require at least start and end colors
    if args.len() < 3 {
        eprintln!("Usage: {} <start_color> <end_color> [steps]", args[0]);
        return;
    }

    // Parse the start and end color codes and optional gradient steps
    let start_color = &args[1];
    let end_color = &args[2];
    let gradient_steps = if args.len() > 3 {
        args[3].parse().unwrap_or(5) // Default to 5 steps if parsing fails
    } else {
        5
    };

    // Generate the color gradient and print each color in the terminal
    match color_gradient(start_color, end_color, gradient_steps) {
        Some(gradient) => {
            for (hex, rgb) in gradient {
                print_color_in_terminal(&hex, rgb);
            }
        }
        None => println!("Invalid hex color input."),
    }
}
