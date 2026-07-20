mod polygon;

use raylib::prelude::*;
use crate::polygon::{draw_polygon, fill_polygon};

fn main() {
    let image_width = 800;
    let image_height = 600;

    let mut image = Image::gen_image_color(image_width, image_height, Color::BLACK);

    let line_color = Color::WHITE;
    let fill3 = Color::new(0xC8, 0x46, 0x46, 255);

    let polygon3: [(i32, i32); 3] = [
        (377, 249), (411, 197), (436, 249),
    ];

    fill_polygon(&mut image, &[&polygon3], fill3);
    draw_polygon(&mut image, &polygon3, line_color);

    let output_file_name = "out.png";
    image.export_image(output_file_name);

    println!("Image saved successfully as '{}'!", output_file_name);
}