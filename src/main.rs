mod polygon;

use raylib::prelude::*;
use crate::polygon::{draw_polygon, fill_polygon};

fn main() {
    let image_width = 800;
    let image_height = 600;

    let mut image = Image::gen_image_color(image_width, image_height, Color::BLACK);

    let line_color = Color::WHITE;
    let fill2 = Color::new(0x3C, 0x8C, 0xDC, 255);

    let polygon2: [(i32, i32); 4] = [
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];

    fill_polygon(&mut image, &[&polygon2], fill2);
    draw_polygon(&mut image, &polygon2, line_color);

    let output_file_name = "out.png";
    image.export_image(output_file_name);

    println!("Image saved successfully as '{}'!", output_file_name);
}