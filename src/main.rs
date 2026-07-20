mod polygon;

use raylib::prelude::*;
use crate::polygon::{draw_polygon, fill_polygon};

fn main() {
    let image_width = 800;
    let image_height = 600;

    let mut image = Image::gen_image_color(image_width, image_height, Color::BLACK);

    let line_color = Color::WHITE;
    let fill1 = Color::new(0xE6, 0xC8, 0x28, 255);

    let polygon1: [(i32, i32); 10] = [
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];

    fill_polygon(&mut image, &[&polygon1], fill1);
    draw_polygon(&mut image, &polygon1, line_color);

    let output_file_name = "out.png";
    image.export_image(output_file_name);

    println!("Image saved successfully as '{}'!", output_file_name);
}