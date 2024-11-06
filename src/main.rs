use std::env;

fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return None;
    }

    let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
    let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
    let b = u8::from_str_radix(&hex[5..7], 16).ok()?;

    Some((r, g, b))
}

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn color_gradient(hex_start: &str, hex_end: &str, steps: usize) -> Option<Vec<(String, (u8, u8, u8))>> {
    let steps = if steps < 2 { 5 } else { steps };

    let (start_r, start_g, start_b) = hex_to_rgb(hex_start)?;
    let (end_r, end_g, end_b) = hex_to_rgb(hex_end)?;

    let mut gradient = Vec::with_capacity(steps);
    for i in 0..steps {
        let ratio = i as f32 / (steps - 1) as f32;

        let r = start_r as f32 + (end_r as f32 - start_r as f32) * ratio;
        let g = start_g as f32 + (end_g as f32 - start_g as f32) * ratio;
        let b = start_b as f32 + (end_b as f32 - start_b as f32) * ratio;

        let color_hex = rgb_to_hex(r.round() as u8, g.round() as u8, b.round() as u8);
        gradient.push((color_hex, (r.round() as u8, g.round() as u8, b.round() as u8)));
    }

    Some(gradient)
}

fn print_color_in_terminal(hex: &str, rgb: (u8, u8, u8)) {
    // ANSI escape code for 24-bit RGB color
    let (r, g, b) = rgb;
    println!("\x1b[48;2;{};{};{}m  \x1b[0m {}", r, g, b, hex);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <start_color> <end_color> [steps]", args[0]);
        return;
    }

    let start_color = &args[1];
    let end_color = &args[2];
    let gradient_steps = if args.len() > 3 {
        args[3].parse().unwrap_or(5)
    } else {
        5
    };

    match color_gradient(start_color, end_color, gradient_steps) {
        Some(gradient) => {
            for (hex, rgb) in gradient {
                print_color_in_terminal(&hex, rgb);
            }
        }
        None => println!("Invalid hex color input."),
    }
}
